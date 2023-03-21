# Axum vs Spring WebFlux (GraalVM)

Relate to this blog post:
 - [TH] [ลองทำ Performance Test ด้วย k6 — Axum vs Spring WebFlux GraalVM Support](https://tpbabparn.medium.com/ลองทำ-performance-test-ด้วย-k6-axum-vs-spring-webflux-graalvm-support-e516603fe1de)
 - [EN] [Testing Performance with k6: Axum vs Spring WebFlux GraalVM Support](https://tpbabparn.medium.com/testing-performance-with-k6-axum-vs-spring-webflux-graalvm-support-62e288204b3a)

Main command for running Test

```bash
docker compose run --rm k6 run /scripts/test.js -u500 -d10s
```
