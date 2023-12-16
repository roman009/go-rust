package com.github.roman009.gorust.externalapi.configuration;

import com.github.roman009.gorust.externalapi.message.Message;
import lombok.extern.log4j.Log4j2;
import org.apache.kafka.clients.admin.AdminClientConfig;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.kafka.config.TopicBuilder;
import org.springframework.kafka.core.KafkaAdmin;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.kafka.support.converter.JsonMessageConverter;
import org.springframework.kafka.support.converter.RecordMessageConverter;

import java.util.HashMap;
import java.util.Map;

@Configuration
@Log4j2
public class KafkaConfiguration {

    @Autowired
    private KafkaTemplate<Object, Object> template;

    @Bean
    public RecordMessageConverter converter() {
        return new JsonMessageConverter();
    }

    @Bean
    public KafkaAdmin admin() {
        Map<String, Object> configs = new HashMap<>();
        configs.put(AdminClientConfig.BOOTSTRAP_SERVERS_CONFIG, "192.168.0.220:9094");
        configs.put(AdminClientConfig.SECURITY_PROTOCOL_CONFIG, "PLAINTEXT");
        return new KafkaAdmin(configs);
    }

    @Bean
    public KafkaAdmin.NewTopics topics456() {
        return new KafkaAdmin.NewTopics(
                TopicBuilder.name("killMessageSentT").build()
        );
    }

    @KafkaListener(id = "messageGroup", topics = "killMessageSentT")
    public void listen(Message message) {
        log.info("Received: " + message);
    }
}
