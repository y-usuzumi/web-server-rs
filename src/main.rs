use webserver::Server;

fn main() {
    let s = Server::new("127.0.0.1", 7878);
    s.run();
}
