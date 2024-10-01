# blockchain-in-rust

blockchain in rust

## project structure

- api/：负责提供外部接口，包括 HTTP API 和 RPC。
- cli/：命令行界面相关代码，实现用户通过终端与程序交互。
- consensus/：实现共识算法，如 PoW（工作量证明）或 PoS（权益证明）。
- core/：区块链的核心逻辑，包括区块、交易和区块链管理。
- network/：处理节点之间的网络通信，实现 P2P 网络。
- crypto/：加密相关的功能，如哈希算法、数字签名等。
- storage/：数据的持久化存储，接口数据库或文件系统。
- utils/：项目中使用的工具函数或公共模块。
