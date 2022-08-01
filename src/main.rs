
use tokio::io::AsyncReadExt;
use tokio::{net::TcpSocket, io::AsyncWriteExt};
use tokio::sync::broadcast::{self};


#[tokio::main]
async fn main() {

    println!(r"
    _          _ _            _   
_ __ _   _ ___| |_    ___| (_) ___ _ __ | |_ 
| '__| | | / __| __|  / __| | |/ _ \ '_ \| __|
| |  | |_| \__ \ |_  | (__| | |  __/ | | | |_ 
|_|   \__,_|___/\__|  \___|_|_|\___|_| |_|\__|");

    let socket = TcpSocket::new_v4().unwrap();
    let addr = "127.0.0.1:5555".parse().unwrap();
    let mut tcp_stream = socket.connect(addr).await.unwrap();
    
    println!("连接成功：{:?}",addr);

    let (tx,mut rx) = broadcast::channel(1024);
    
    //这个地方是因为下面线程中使用后他自动推导不出类型，所以在这个地方添加一个空的字符串
    tx.send(String::new()).unwrap();
    //开启线程处理消息和给服务端发送消息
    tokio::spawn(async move {
        loop {
            let (mut read_half,mut write_half) = tcp_stream.split();
            let mut buf = vec![0;1024];
            tokio::select! {
                //读取服务端的消息
                result = read_half.read_buf(&mut buf) => {
                    match result {
                        Ok(num) => {
                            //这个地方是因为每次接收到消息，总是会出现空消息，如果接收到不是空消息就不是1024大小
                            if num != 1024 {
                                //转换字符
                                let content = String::from_utf8(buf).unwrap();
                                //输出消息
                                print!("size = {},receive = {}",num,content);
                            }
                        }
                        Err(err) => {
                            println!("服务器连接断开！err={:?}",err);
                            //如果服务端断开则退出程序
                            std::process::exit(0);
                        }
                    }
                    
                }
                //读取通道的消息发送给服务端
                result = rx.recv() => {
                    let send = result.unwrap();
                    //发送给服务端
                    let _ = write_half.write_all(send.as_bytes()).await;
                    let _ = write_half.flush();
                }
            }
        }
    });
    loop {
        //创建string
        let mut send = String::new();
        //读取控制台文字
        std::io::stdin().read_line(&mut send).unwrap();
        //通过通道发送
        tx.send(send).unwrap();
    }
    
    
    
}
