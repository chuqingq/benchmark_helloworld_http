from fastapi import FastAPI
from fastapi.responses import PlainTextResponse
import uvicorn

# 创建一个FastAPI应用程序实例
app = FastAPI()
# 定义路由（使用装饰器将函数绑定到特定的路径和HTTP方法）
@app.get("/hello", response_class=PlainTextResponse)
async def root():
    return "hello world"

# 启动程序时使用 uvicorn 允许 FastAPI 应用程序
uvicorn.run(app, port=8081, access_log=False)
# 默认ip为127.0.0.1，默认端口为8000
