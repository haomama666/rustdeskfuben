// 编译时服务器配置模块
// 此模块用于在编译时设置RustDesk客户端的服务器配置，使客户端默认连接到指定服务器
// 并限制用户修改这些配置
use hbb_common::{lazy_static, config}; // 导入必要的库

// 定义编译时配置结构体
// 存储所有需要在编译时设置的服务器配置信息
pub struct CompileTimeConfig {
    pub rendezvous_server: String, // 服务器地址（用于P2P连接和中继服务器发现）
    pub relay_server: String,      // 中继服务器地址（当P2P连接失败时使用）
    pub api_server: String,        // API服务器地址（用于用户认证和管理功能）
    pub key: String,               // 服务器公钥（用于加密通信和验证服务器身份）
}

// ======================================================
// 以下是您需要根据自己的服务器信息修改的常量
// 修改这些值将直接影响编译后客户端的默认服务器设置
// ======================================================

// 自定义服务器地址（必填）
// 修改效果：客户端启动时会自动连接到此服务器
// 格式：服务器域名或IP地址:端口号（默认端口为21117）
const CUSTOM_RENDEZVOUS_SERVER: &str = "123.56.52.21:21117";

// 自定义中继服务器地址（可选）
// 修改效果：设置客户端使用的中继服务器，若留空则使用服务器自动分配的中继
// 格式：服务器域名或IP地址:端口号（默认端口为21117）
const CUSTOM_RELAY_SERVER: &str = "123.56.52.21:21117";

// 自定义API服务器地址（可选）
// 修改效果：设置客户端连接的API服务器，用于用户认证和管理功能
// 格式：服务器域名或IP地址:端口号（默认端口为21114）
const CUSTOM_API_SERVER: &str = "http://123.56.52.21:21114";

// 自定义服务器公钥（必填）
// 修改效果：设置用于加密通信的服务器公钥，确保连接安全性
// 格式：Base64编码的公钥字符串
const CUSTOM_KEY: &str = "I+4iSpQm+RRTCxCTiK2rIbPqNs5fTcEatxI9UBmWuqE="; // 用户要求添加的密码作为密钥

// ======================================================
// 以下部分为配置初始化和管理逻辑，一般不需要修改
// ======================================================

// 使用lazy_static创建全局配置实例
// 这样配置只在第一次访问时初始化，提高性能
lazy_static::lazy_static! {
    // 全局编译时配置实例，存储所有服务器设置
    pub static ref COMPILE_TIME_CONFIG: CompileTimeConfig = CompileTimeConfig {
        rendezvous_server: CUSTOM_RENDEZVOUS_SERVER.to_string(),
        relay_server: CUSTOM_RELAY_SERVER.to_string(),
        api_server: CUSTOM_API_SERVER.to_string(),
        key: CUSTOM_KEY.to_string(),
    };
    
    // 确保配置只初始化一次的标志
    // 使用RwLock保证在多线程环境下的安全访问
    static ref INITIALIZED: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(false));
}

// 初始化编译时配置的函数
// 此函数会将编译时配置应用到RustDesk的全局配置系统中
pub fn init_compile_time_config() {
    // 获取编译时配置实例
    let config = &COMPILE_TIME_CONFIG;
    
    // 设置服务器地址
    // 修改效果：覆盖默认的服务器地址，客户端将连接到此处设置的服务器
    if !config.rendezvous_server.is_empty() {
        config::set(config::keys::OPTION_CUSTOM_RENDEZVOUS_SERVER, &config.rendezvous_server);
    }
    
    // 设置中继服务器地址
    // 修改效果：指定用于中继连接的服务器，当P2P连接失败时使用
    if !config.relay_server.is_empty() {
        config::set(config::keys::OPTION_RELAY_SERVER, &config.relay_server);
    }
    
    // 设置API服务器地址
    // 修改效果：指定用于用户认证和管理功能的API服务器
    if !config.api_server.is_empty() {
        config::set(config::keys::OPTION_API_SERVER, &config.api_server);
    }
    
    // 设置密钥
    // 修改效果：设置用于加密通信的公钥，确保连接的安全性
    if !config.key.is_empty() {
        config::set(config::keys::OPTION_KEY, &config.key);
    }
    
    // 隐藏连接管理窗口
    // 修改效果：允许隐藏连接管理窗口，提供更简洁的用户界面
    config::set("allow-hide-cm", "Y");
    
    // 允许远程配置修改
    // 修改效果：允许远程设备修改当前客户端的配置
    config::set(config::keys::OPTION_ALLOW_REMOTE_CONFIG_MODIFICATION, "Y");
    
    // 设置默认连接密码
    // 修改效果：设置远程连接时使用的默认密码
    config::set(config::keys::RELAY_PASS, "Mm118811");
    
    // 设置完全控制权限
    // 修改效果：授予远程连接完全控制权限
    config::set(config::keys::OPTION_PERMISSION, "all");
    
    // 以下代码将服务器配置添加到HARD_SETTINGS中，防止用户在界面上修改这些设置
    // 修改效果：锁定服务器相关设置，用户无法在客户端界面更改这些配置
    config::HARD_SETTINGS
        .write()
        .unwrap()
        .insert(config::keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_string(), config.rendezvous_server.clone());
    
    config::HARD_SETTINGS
        .write()
        .unwrap()
        .insert(config::keys::OPTION_RELAY_SERVER.to_string(), config.relay_server.clone());
    
    config::HARD_SETTINGS
        .write()
        .unwrap()
        .insert(config::keys::OPTION_API_SERVER.to_string(), config.api_server.clone());
    
    config::HARD_SETTINGS
        .write()
        .unwrap()
        .insert(config::keys::OPTION_KEY.to_string(), config.key.clone());
    
    // 设置为自定义客户端
    // 修改效果：更改客户端的显示名称，表明这是一个定制版本
    *config::APP_NAME.write().unwrap() = "RustDesk Custom";
}

// 公共初始化函数，确保配置只被初始化一次
// 使用双重检查锁定模式，确保线程安全
pub fn ensure_compile_time_config_initialized() {
    // 读取锁检查是否已经初始化
    // 这样可以在多线程环境中高效地检查初始化状态
    if !*INITIALIZED.read().unwrap() {
        // 获取写入锁，准备进行初始化
        let mut initialized = INITIALIZED.write().unwrap();
        // 双重检查，防止多线程竞态条件
        // 确保即使在获取写入锁的过程中有其他线程完成了初始化，也不会重复执行
        if !*initialized {
            init_compile_time_config(); // 执行初始化
            *initialized = true;        // 标记为已初始化
        }
    }
}