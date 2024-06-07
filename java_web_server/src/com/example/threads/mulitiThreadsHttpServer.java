package java_web_server.src.com.example.threads;

import java.io.IOException;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

public class mulitiThreadsHttpServer {
    public static void main(String[] args) throws IOException {
        HttpServer server = HttpServer.create(new InetSocketAddress(3000), 0);

        server.createContext("/", new HelloHandler());

        int poolSize = 2 * Runtime.getRuntime().availableProcessors();
        ExecutorService executor = Executors.newFixedThreadPool(poolSize);

        server.setExecutor(executor);

        server.start();
    }

    static class HelloHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            // This is where you handle the incoming HTTP request
            String response = "Hello, World!";
            exchange.sendResponseHeaders(200, response.length());
            OutputStream os = exchange.getResponseBody();
            os.write(response.getBytes());
            os.close();
        }
    }
}
