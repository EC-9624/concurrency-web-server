package java_web_server.src.com.example.virtualThreads;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

public class virtualThreadHttpServer {

    private static String distPath;

    public static void main(String[] args) throws IOException {

        Path currentRelativePath = Paths.get("");
        String currentDir = currentRelativePath.toAbsolutePath().toString();
        distPath = Paths.get(currentDir, "dist").toString();

        HttpServer server = HttpServer.create(new InetSocketAddress(3000), 0);

        server.createContext("/", new StaticFileHandler());

        // Create an ExecutorService that uses virtual threads
        ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor();

        server.setExecutor(executor);
        server.start();

        System.out.println("Server is listening on port 3000");
    }

    static class StaticFileHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            String requestPath = exchange.getRequestURI().getPath();
            File file = new File(distPath, requestPath);

            if (file.isDirectory()) {
                file = new File(file, "index.html");
            }

            if (!file.exists()) {
                // Serve index.html for client-side routing
                file = new File(distPath, "index.html");
            }

            byte[] fileBytes = readFile(file);

            String contentType = getContentType(file.getName());

            // Send response
            exchange.getResponseHeaders().set("Content-Type", contentType);
            exchange.sendResponseHeaders(200, fileBytes.length);
            OutputStream os = exchange.getResponseBody();
            os.write(fileBytes);
            os.close();
        }

        private byte[] readFile(File file) throws IOException {
            FileInputStream fis = new FileInputStream(file);
            byte[] data = new byte[(int) file.length()];
            fis.read(data);
            fis.close();
            return data;
        }

        private String getContentType(String fileName) {
            String ext = fileName.substring(fileName.lastIndexOf(".") + 1);
            Map<String, String> mimeTypes = new HashMap<>();
            mimeTypes.put("html", "text/html");
            mimeTypes.put("js", "application/javascript");
            mimeTypes.put("css", "text/css");
            mimeTypes.put("json", "application/json");
            mimeTypes.put("png", "image/png");
            mimeTypes.put("jpg", "image/jpeg");
            mimeTypes.put("jpeg", "image/jpeg");
            mimeTypes.put("gif", "image/gif");
            mimeTypes.put("svg", "image/svg+xml");
            mimeTypes.put("wav", "audio/wav");
            mimeTypes.put("mp4", "video/mp4");
            mimeTypes.put("woff", "application/font-woff");
            mimeTypes.put("ttf", "application/font-ttf");
            mimeTypes.put("eot", "application/vnd.ms-fontobject");
            mimeTypes.put("otf", "application/font-otf");
            mimeTypes.put("wasm", "application/wasm");
            return mimeTypes.getOrDefault(ext, "application/octet-stream");
        }
    }
}
