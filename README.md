# gRPC-Client-Server-Rust

## gRPC - open source universal RPC framework

![alt text](https://res.cloudinary.com/practicaldev/image/fetch/s--nhFKbrqG--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/v4vvjnbirspyrhh6usaf.png)

**gRPC** is a modern open source high performance Remote Procedure Call (RPC) framework that can run in any environment. It can efficiently connect services in and across data centers with pluggable support for load balancing, tracing, health checking and authentication. It is also applicable in last mile of distributed computing to connect devices, mobile applications and browsers to backend services.

## Case Study

The serving process, called **Server**, waits to receive messages from a group of 3 processes called **Clients**. Each client sends 5 processing requests. Each message sent must contain the Client's PID and two integer values, randomly selected between 0 and 100. The Client processes must also wait and print on the screen the response message from the Server, before sending the next request. Client processes must be generated by the main program itself. When all Clients terminate, the parent process sends a special message to the Server, containing the pair of values {-1, -1}, which causes the Server to terminate.

The Server processes the received messages as follows: it assigns the message to a thread, which extracts the pair of values and the PID of the process. Each thread calculates the product of the pair of input values, and has the task of sending the Client a message containing the calculated value, after which the thread ends. When the Server receives the message with the pair of values **{-1, -1}**, it ends.

![alt text](https://raw.githubusercontent.com/Aledangelo/gRPC-Client-Server-Rust/main/deployment_diagram.png)