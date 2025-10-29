use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

use ahash::{AHashMap, AHashSet};
use anyhow::Ok;
use handler::ShareableMessageHandler;
use matching::is_matching_backtracking;
use indexmap::IndexMap;
use ant_core::{
    correctness::{check_predicate_true, check_valid_string, FAILED}, UUID4
};
use log::kv::value;
use ustr::Ustr;
use super::{handler, matching, set_message_bus, switchboard::MessagingSwitchboard};
use ant_model::{data::DataType, identifiers::trade_id};
use ant_model::identifiers::TraderId;

#[inline(always)]
fn check_fully_qualified_string(value:&Ustr, key: &str) -> anyhow::Result<()>{
    check_predicate_true(
        !value.chars().any(|c| c == '*' || c == '?'),
        &format!("{key} `value` contained invalid characters, was {value}"),
    )
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pattern;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Endpoint;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MStr<T>{
    value: Ustr, 
    _maker: std::marker::PhantomData<T>,
}

impl<T> Display for MStr<T> {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.value)
    }
}

impl<T> Deref for MStr<T>{
    type Target = Ustr;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}


impl MStr<Pattern> {
    // 使用一个 string 创建 一个 Pattern
    pub fn pattern<T: AsRef<str>>(value: T) -> Self {
        let value = Ustr::from(value.as_ref());

        Self { value: (value), _maker: (std::marker::PhantomData) }
    }
}

impl <T: AsRef<str>> From<T> for MStr<Pattern> {
    // 两种调用方式：
    // 1. 显式：MStr::<Pattern>::from(value)
    // 2. 隐式：通过 Into 自动转换（如 value.into()）
    fn from(value: T) -> Self{
        Self::pattern(value)
    }
}

impl From<MStr<Topic>> for MStr<Pattern> {
    // 使用 Topic 类型转成 Pattern 
    fn from(value: MStr<Topic>) -> Self {
        Self {
            value: value.value,
            _maker: std::marker::PhantomData,
        }
    }
}

impl MStr<Topic> {
    pub fn topic<T: AsRef<str>>(value: T) -> anyhow::Result<Self>{
        let topic = Ustr::from(value.as_ref());
        check_valid_string(value, stringify!(value))?;
        check_fully_qualified_string(&topic, stringify!(Topic))?;

        Ok(Self{
            value: topic,
            _maker: std::marker::PhantomData
        })
    }
}

impl<T: AsRef<str>> From<T> for MStr<Topic>{
    fn from(value: T) -> Self{
        Self::topic(value).expect(FAILED)
    }
}

impl MStr<Endpoint> {
    pub fn endpoint<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let endpoint = Ustr::from(value.as_ref());
        check_valid_string(value, stringify!(value))?;
        check_fully_qualified_string(&endpoint, stringify!(Endpoint))?;

        Ok(Self {
            value: endpoint,
            _maker: std::marker::PhantomData,
        })
    }
}


impl<T: AsRef<str>> From<T> for MStr<Endpoint> {
    fn from(value: T) -> Self {
        Self::endpoint(value).expect(FAILED)
    }
}

// 订阅一个主题
#[derive(Clone, Debug)]
pub struct Subscription{
    // 消息接收处理方法
    pub hanlder: ShareableMessageHandler,
    //存储hanlder Id 用于快速的比较检查
    pub handler_id: Ustr,

    // 订阅消息的模式
    pub pattern: MStr<Pattern>,

    // 订阅的优先级
    pub priority: u8,
}

impl Subscription{
    //创建订阅实例
    #[must_use]
    pub fn new(
        pattern: MStr<Pattern>,
        handler: ShareableMessageHandler,
        priority: Option<u8>,
    ) -> Self{
        Self { 
            handler_id: handler.0.id(), 
            hanlder: handler,
            pattern, 
            priority: priority.unwrap_or(0),
        }
    }
}

// 判断订阅是否一致， 需要模式一致，且 定义的唯一ID一致
impl PartialEq<Self> for Subscription{
    fn eq(&self, other:&Self) -> bool{
        self.pattern == other.pattern && self.handler_id == other.handler_id
    }
}

impl Eq for Subscription {}

//最终使得 Subscription 类型可以参与排序操作（如 <、>、<=、>= 等比较）。
impl PartialOrd for Subscription{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Subscription {
    //// 排序逻辑通过三个层级的比较实现，
    /// 核心是优先按 priority（优先级）降序排列，
    /// 优先级相同则按 pattern 升序，
    /// 最后按 handler_id 升序。
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.pattern.cmp(&other.pattern))
            .then_with(|| self.handler_id.cmp(&other.handler_id))
    }
}

impl Hash for Subscription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
        self.handler_id.hash(state);
    }
}

//// 通用的消息总线 满足各种消息模式
//// 
//// 总线提供生产者和消费者 API, 提供 发布/订阅， 请求/响应， 也有直接点对点发送消息
//// Pub/Sub 发布 - 订阅（Pub/Sub）系统 中，用于 层级化主题（hierarchical topics） 的通配符模式规则，具体定义了两种通配符的含义：
////     *（星号）：表示匹配一个或多个字符（在同一层级内）。例如，若主题是层级化的（通常用 . 分隔层级，如 a.b.c），a.*.c 可匹配 a.x.c、a.xy.c 等（* 匹配了 x 或 xy 等多个字符）。
////     ?（问号）：表示匹配单个字符（在同一层级内）。例如，a.?.c 可匹配 a.x.c、a.1.c 等（? 仅匹配一个字符 x 或 1）。
////    假设主题格式为 层级1.层级2.层级3（如 news.sport.football）：
////     news.*.football 可匹配 news.sport.football、news.local.football（* 匹配 sport 或 local 等多个字符）。
////     news.sport.? 可匹配 news.sport.f、news.sport.1（? 匹配单个字符 f 或 1）。
////     *.?.? 可匹配 a.b.c、x.1.2（第一级任意多个字符，后两级各一个字符）

#[derive(Debug)]
pub struct MessageBus{
    // 交易ID 关联到这个bus
    pub trader_id: TraderId,

    pub instance_id: UUID4,

    pub name: String,

    pub has_backing: bool,

    pub switchboard: MessagingSwitchboard,

    pub subscriptions: AHashSet<Subscription>,

    pub topics: IndexMap<MStr<Topic>, Vec<Subscription>>,

    pub endpoints: IndexMap<MStr<Endpoint>, ShareableMessageHandler>,

    pub correlation_index: AHashMap<UUID4, ShareableMessageHandler>,
}

impl MessageBus {
    #[must_use]
    pub fn new(
        trader_id: TraderId,
        instance_id: UUID4,
        name: Option<String>,
        _config: Option<Option<HashMap<String, serde_json::Value>>>,
    ) -> Self{
        Self{
            trader_id,
            instance_id,
            name: name.unwrap_or(stringify!(MessageBus).to_owned()),
            switchboard: MessagingSwitchboard::default(),
            subscriptions: AHashSet::new(),
            topics: IndexMap::new(),
            endpoints:IndexMap::new(),
            correlation_index: AHashMap::new(),
            has_backing: false,
        }
    }

    #[must_use]
    pub fn mem_address(&self) -> String{
        format!("{self:p}")
    }

    #[must_use]
    pub fn endpoints(&self) -> Vec<&str> {
        self.endpoints
            .iter()
            .map(|e|e.0.as_str())
            .collect()
    }

    #[must_use]
    pub fn patterns(&self) -> Vec<&str>{
        self.subscriptions
            .iter()
            .map(|e|e.pattern.as_str())
            .collect()
    }

    #[must_use]
    pub fn has_subscribers<T: AsRef<str>>(&self, topic: T) -> bool{
        self.subscriptions_count(topic) > 0
    }
    
    pub(crate) fn find_topic_matches(&self, topic: MStr<Topic>) -> Vec<Subscription>{
        self.subscriptions
            .iter()
            .filter_map(|sub|{
                if is_matching_backtracking(topic, sub.pattern){
                    Some(sub.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    #[must_use]
    pub fn subscriptions_count<T: AsRef<str>>(&self, topic: T) -> usize{
        let topic = MStr::<Topic>::topic(topic).expect(FAILED);
        self.topics
            .get(&topic)
            .map(|subs| subs.len())
            .unwrap_or_else(|| self.find_topic_matches(topic).len())
    }

    #[must_use]
    pub fn subscriptions(&self) -> Vec<&Subscription>{
        self.subscriptions
            .iter()
            .collect()
    }

    #[must_use]
    pub fn subscription_handler_ids(&self) -> Vec<&str>{
        self.subscriptions
            .iter()
            .map(|s| s.handler_id.as_str())
            .collect()
    }

    #[must_use]
    pub fn is_registered<T: AsRef<str>>(&self, endpoint: T) -> bool{
        let endpoint: MStr<Endpoint> = endpoint.into();
        self.endpoints.contains_key(&endpoint)
    }

    #[must_use]
    pub fn is_subscribed<T: AsRef<str>>(
        &self,
        pattern: T,
        handler: ShareableMessageHandler,
    ) -> bool {
        let pattern = MStr::<Pattern>::pattern(pattern);
        let sub = Subscription::new(pattern, handler, None);
        self.subscriptions.contains(&sub)
    }

    pub fn close(&self) -> anyhow::Result<()> {
        // TODO: Integrate the backing database
        Ok(())
    }

    #[must_use]
    pub fn get_endpoint(&self, endpoint: MStr<Endpoint>) -> Option<&ShareableMessageHandler>{
        self.endpoints.get(&endpoint)
    }

    #[must_use]
    pub fn get_response_handler(&self, correlation_id: &UUID4) -> Option<&ShareableMessageHandler>{
        self.correlation_index.get(correlation_id)
    }

    #[must_use]
    pub fn matching_subscriptions<T: AsRef<str>>(&mut self, topic: T) -> Vec<Subscription> {
        let topic = MStr::<Topic>::from(topic);
        self.inner_matching_subscriptions(topic)
    }

    pub(crate) fn inner_matching_subscriptions(&mut self, topic:MStr<Topic>) -> Vec<Subscription>{
        self.topics.get(&topic).cloned().unwrap_or_else(||{
            let mut matchs = self.find_topic_matches(topic);
            matchs.sort();
            self.topics.insert(topic, matchs.clone());
            matchs
        })
    }

    pub fn register_response_handler(
        &mut self,
        correlation_id: &UUID4,
        handler: ShareableMessageHandler,
    ) -> anyhow::Result<()>{
        if self.correlation_index.contains_key(correlation_id){
            anyhow::bail!("Correlaton Id<{correlation_id}> already has a register hanlder");
        }
        self.correlation_index.insert(*correlation_id, handler);
        Ok(())
    }
}

impl MessageBus{
    pub fn register_message_bus(self) -> Rc<RefCell<MessageBus>>{
        let msgbus = Rc::new(RefCell::new(self));
        set_message_bus(msgbus.clone());
        msgbus
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new(TraderId::from("TRADER-001"), UUID4::new(), None, None)
    }
}
