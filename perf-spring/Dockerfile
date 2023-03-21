FROM ghcr.io/graalvm/graalvm-ce:22.3.1

WORKDIR /app
ADD . /app

RUN gu install native-image
RUN native-image --version
RUN ./gradlew nativeCompile


# Multistage
FROM oraclelinux:9-slim
MAINTAINER Thanaphoom Babparn
COPY --from=0 /app/build/native/nativeCompile/perf-test /app/perf-test
CMD ["sh", "-c", "/app/perf-test"]