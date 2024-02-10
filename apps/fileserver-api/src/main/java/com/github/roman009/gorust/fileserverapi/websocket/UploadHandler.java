package com.github.roman009.gorust.fileserverapi.websocket;

import lombok.extern.log4j.Log4j2;
import org.apache.logging.log4j.util.Strings;
import org.springframework.web.socket.BinaryMessage;
import org.springframework.web.socket.SubProtocolCapable;
import org.springframework.web.socket.WebSocketSession;
import org.springframework.web.socket.handler.BinaryWebSocketHandler;

import java.text.MessageFormat;
import java.util.List;
import java.util.Objects;

@Log4j2
public class UploadHandler extends BinaryWebSocketHandler implements SubProtocolCapable {

    @Override
    public boolean supportsPartialMessages() {
        return true;
    }

    @Override
    public void afterConnectionEstablished(WebSocketSession session) throws Exception {
        log.info(MessageFormat.format(
                "Connection established with session: {0} from: {1}",
                session.getId(),
                Objects.isNull(session.getRemoteAddress()) ? session.getRemoteAddress().getHostName() : session.getRemoteAddress().getAddress().getHostAddress())
        );
        super.afterConnectionEstablished(session);
    }

    @Override
    protected void handleBinaryMessage(WebSocketSession session, BinaryMessage message) throws Exception {
        // accept file and save it to ./uploads. check if the file has multiple parts and if so, save them all
        // to a temporary file and then merge them into one file
        // for every 100KB of file transfer done, send a message to the client with the progress
        log.info(MessageFormat.format(
                "Received file: {0} on session: {1} from: {2}",
                message.getPayload().array().length,
                session.getId(),
                Objects.isNull(session.getRemoteAddress()) ? session.getRemoteAddress().getHostName() : session.getRemoteAddress().getAddress().getHostAddress())
        );
        if (message.isLast()) {
            log.info(MessageFormat.format(
                    "File transfer completed on session: {0} from: {1}",
                    session.getId(),
                    Objects.isNull(session.getRemoteAddress()) ? session.getRemoteAddress().getHostName() : session.getRemoteAddress().getAddress().getHostAddress())
            );
        }
    }

    @Override
    public List<String> getSubProtocols() {
        return List.of("binary");
    }
}
