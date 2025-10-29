use std::future;

use bytes::Bytes;
use futures::stream::Stream;
use ustr::Ustr;

use super::message::BusMessage;

#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.common")
)]
#[derive(Debug)]
pub struct MessageBusListener{
    tx: tokio::sync::mpsc::UnboundedSender<BusMessage>,
    rx: Option<tokio::sync::mpsc::UnboundedReceiver<BusMessage>>,
}

impl MessageBusListener {
    /// 创建消息监听器
    pub fn new() ->Self{
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<BusMessage>();
        Self{ tx, rx: Some(rx)}
    }

    /// 监听是否关闭
    pub fn is_closed(&self) -> bool{
        self.tx.is_closed()
    }

    /// 关闭监听器
    pub fn close(&mut self) {
        log::debug!("Closing");

        if let Some(rx) = self.rx.take(){
            drop(rx);
        }

        let (new_tx, _) = tokio::sync::mpsc::unbounded_channel();
        /// 调用新 tx 的 send 方法时，会因新队列的 rx 未被使用而返回 Err（消息被丢弃），
        /// 但不会导致 panic（unbounded_channel 的 send 失败是正常错误，而非 panic）。
        let _ = std::mem::replace(&mut self.tx, new_tx);

        log::debug!("Closed");
    }

    // 发布一个消息
    pub fn publish(&self, topic: Ustr, payload: Bytes){
        let msg = BusMessage::new(topic, payload);
        if let Err(e) = self.tx.send(msg) {
            log::error!("Failed to send message:{e}");
        }
    }

    pub fn get_stream_receiver(
        &mut self,
    ) -> anyhow::Result<tokio::sync::mpsc::UnboundedReceiver<BusMessage>>{
        self.rx
            .take()
            .ok_or_else(|| anyhow::anyhow!("Stream receiver already taken"))
    }

    pub fn stream(
        mut stream_rx: tokio::sync::mpsc::UnboundedReceiver<BusMessage>
    ) -> impl Stream<Item = BusMessage> + 'static{
        async_stream::stream!{
            while let Some(msg) = stream_rx.recv().await{
                yield msg;
            }
        }
    }
}

impl Default for MessageBusListener{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests{
    use bytes::Bytes;
    use futures::StreamExt;
    use super::*;

    #[tokio::test]
    async fn test_new_listener(){
        let listener = MessageBusListener::new();
        assert!(!listener.is_closed())
    }

    #[tokio::test]
    async fn test_close_listener(){
        let mut listner = MessageBusListener::new();
        listner.close();
        assert!(listner.is_closed())
    }

    #[tokio::test]
    async fn test_publish_and_receive(){
        let mut listener = MessageBusListener::new();
        
        let rx = listener
            .get_stream_receiver()
            .expect("Faild to get stream receiver");

        let (notify_tx, mut notify_rx) = tokio::sync::mpsc::channel::<()>(1);

        let handle = tokio::spawn(async move{
            let stream = MessageBusListener::stream(rx);
            futures::pin_mut!(stream);
            let msg = stream.next().await.expect("No message received");
            assert_eq!(msg.topic, "test-topic");
            assert_eq!(msg.payload.as_ref(), b"test-payload");
            notify_tx.send(()).await.unwrap();
        });
        listener.publish(Ustr::from("test-topic"), Bytes::from("test-payload"));
        tokio::select! {
            _ = notify_rx.recv() => {},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(100)) => {
                panic!("Timeout waiting for message");
            }
        }
         // Clean up
         handle.await.unwrap();
    }

        
    #[tokio::test]
    async fn test_multiple_messages(){
        let mut listener = MessageBusListener::new();
        let rx = listener
            .get_stream_receiver()
            .expect("Failed to get stream receiver");

        let topics = vec!["topic1", "topic2", "topic3"];
        let payloads = vec!["payload1", "payload2", "payload3"];

        let topics_clone = topics.clone();
        let payloads_clone = payloads.clone();
        
        let handle = tokio::spawn(async move{
            let stream = MessageBusListener::stream(rx);
            futures::pin_mut!(stream);
            let mut received = Vec::new();
            // 接受消息 
            for _ in 0..3{
                if let Some(msg) = stream.next().await{
                    received.push((msg.topic, String::from_utf8(msg.payload.to_vec()).unwrap()));
                }
            }

            // Verify all messages were received
            for i in 0..3 {
                assert!(
                    received
                        .contains(&(Ustr::from(topics_clone[i]), payloads_clone[i].to_string()))
                );
            }

            received
        });

        // 发送消息
        for i in 0..3{
            listener.publish(
                Ustr::from(topics[i]),
                Bytes::from(payloads[i].as_bytes().to_vec())
            );
        }

        let result = tokio::time::timeout(tokio::time::Duration::from_secs(1), handle)
            .await
            .expect("Test timed out")
            .expect("Task panicked");

        assert_eq!(result.len(), 3);

    }

    #[tokio::test]
    async fn test_stream_receiver_already_taken(){
        let mut listener = MessageBusListener::new();
        let _rx = listener
            .get_stream_receiver()
            .expect("Faild to get stream reciever");

        assert!(listener.get_stream_receiver().is_err());
    }

    #[tokio::test]
    async fn test_publish_after_close() {
        let mut listener = MessageBusListener::new();

        let _rx = listener
            .get_stream_receiver()
            .expect("Faild to get stream reciever");

        listener.close();
        assert!(listener.is_closed());
        listener.publish(Ustr::from("test-topic"), Bytes::from("test-payload"));
    }
}
