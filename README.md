# 早安机器人
## 功能
### 早安问候
每天早晨，自动获取当前天气，并根据天气情况生成早安问候，同时获取一句古诗词，根据古诗词的内容生成一张图片，发送到企业微信群中。

![](https://vip2.loli.io/2023/08/11/fRhNGXwiC1jETlZ.webp)

## 使用
直接使用打包的二进制文件即可，需要提供一下环境变量：
* CHATGPT_TOKEN: chatgpt 的 token
* MIDJOURNEY_PROXY_RUL: midjourney 的代理地址，参见项目 [midjourney-proxy](https://github.com/novicezk/midjourney-proxy)
* MIDJOURNEY_PROXY_SECRET: midjourney 的代理密钥
* WECHAT_BOT_URL: 企业微信群机器人的 webhook 地址
* SMMS_TOKEN: sm.ms 图床的 token，可选
