# pi_futures

提供BoxFuture，根据条件不同，它能为下面两种类型：
## Pin<Box<dyn Future<Output = T> + 'a>>
wasm平台使用该类型, 或可打开feature: `local`

## Pin<Box<dyn Future<Output = T> + Send + 'a>> 
不是wasm平台，并且未添加feature：`local`