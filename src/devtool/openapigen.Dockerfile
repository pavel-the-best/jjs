FROM maven:3.6.3-openjdk-15 as builder
RUN yum install -y wget tar gzip maven
RUN wget https://github.com/mikailbag/openapi-generator/archive/873a4791bd7309b81fbad66f5c347baed0533617.tar.gz -O gen.tgz 
RUN mkdir /gen && tar xzf  gen.tgz --directory /gen
WORKDIR /gen
RUN mv $( ls )/* .
RUN mvn package -DskipTests=true

FROM openjdk:jre
COPY --from=builder /gen/modules/openapi-generator-cli/target/openapi-generator-cli.jar openapi-generator-cli.jar
ENTRYPOINT ["java", "-jar", "/openapi-generator-cli.jar"]