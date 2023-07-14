fn main() {
    println!("Hello, world!");
}

// use tokio to execute the function asynchronously
//bind to a port and listen to all incoming connections on all network interfaces
// initialise the ip address of the backend servers to hit
//start a loop listening for incoming conn.
//on receiving a conn. stream , randomly select a server and forward the request 
// use tokio::spawn to spawn a new asynchronous task for each connection. Inside the task, establish a connection to the chosen backend server using TcpStream::connect.
//split the client and backend streams into separate reader and writer halves using into_split.
//use tokio::io::copy to asynchronously copy data between the client and backend streams. This allows us to forward incoming data from the client to the backend server, and vice versa.