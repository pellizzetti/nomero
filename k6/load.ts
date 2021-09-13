import http from "k6/http";
import { sleep, check } from "k6";
import { Counter } from "k6/metrics";

const randomInput = (min, max) =>
  Math.floor(Math.random() * (max - min + 1) + min);

export const requests = new Counter("http_reqs");

export const options = {
  stages: [
    { duration: "30s", target: 550 },
    { duration: "1m", target: 550 },
    { duration: "18s", target: 1000 },
    { duration: "12s", target: 1000 },
    { duration: "18s", target: 550 },
    { duration: "1m", target: 550 },
    { duration: "30s", target: 0 },
  ],
  thresholds: {
    http_req_duration: ["p(99)<1500"], // 99% of requests must complete below 1.5s
  },
};

export default function () {
  const input = randomInput(1, 92);
  const res = http.get(`https://nomero.rocketseat.dev/api/fibonacci/${input}`);

  sleep(1);

  check(res, {
    "status is 200": (r) => r.status === 200,
  });
}
