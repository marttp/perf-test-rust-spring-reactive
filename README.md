# Axum vs Spring WebFlux (GraalVM)

Relate to this blog post: [ลองทำ Performance Test ด้วย k6 — Axum vs Spring WebFlux GraalVM Support](https://tpbabparn.medium.com/ลองทำ-performance-test-ด้วย-k6-axum-vs-spring-webflux-graalvm-support-e516603fe1de)

Main command for running Test

```bash
docker compose run --rm k6 run /scripts/test.js -u500 -d10s
```
