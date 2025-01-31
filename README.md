


## Setup + Run
Prerequisites:
- Rust
- oha load test [download 1.3.0](https://github.com/hatoo/oha/releases/tag/v1.3.0)

```bash
RUST_LOG=info cargo run --release -- --router-base-api-url https://router.example.com/
```


```bash
# use limit for faster testing roundtrip
RUST_LOG=info cargo run -- --router-base-api-url https://router.example.com/ --limit 10
```

## Examples

```csv
input_mint,output_mint,status_200,latency_p99
MNDEFzGvMt87ueuHvVU9VcTqsAP5b3fTGPsHuuPA5ey,So11111111111111111111111111111111111111112,40,330.13
nosXBVoaCTtYdLvKY6Csb4AC8JCdQKKAaWYtx2ZMoo7,So11111111111111111111111111111111111111112,37,418.21
45EgCwcPXYagBC7KqBin4nCFgEZWN7f3Y6nACwxqMCWX,So11111111111111111111111111111111111111112,40,366.22
A1KLoBrKBde8Ty9qtNQUtq3C2ortoC3u7twggz7sEto6,So11111111111111111111111111111111111111112,38,328.59
BqVHWpwUDgMik5gbTciFfozadpE2oZth5bxCDrgbDt52,So11111111111111111111111111111111111111112,40,366.85
7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU,So11111111111111111111111111111111111111112,45,324.27
METADDFL6wWMWEoKTFJwcThTbUmtarRJZjRpzUvkxhr,So11111111111111111111111111111111111111112,43,336.72
6DSqVXg9WLTWgz6LACqxN757QdHe1sCqkUfojWmxWtok,So11111111111111111111111111111111111111112,42,342.95

```

```json
{
  "details": {
    "DNSDialup": {
      "average": 0.2863108125,
      "fastest": 0.279199333,
      "slowest": 0.293422292
    },
    "DNSLookup": {
      "average": 0.000116229,
      "fastest": 0.000100667,
      "slowest": 0.000131791
    }
  },
  "errorDistribution": {
    "aborted due to deadline": 2
  },
  "latencyPercentiles": {
    "p10": 0.025709833,
    "p25": 0.027431167,
    "p50": 0.029952542,
    "p75": 0.066810875,
    "p90": 0.441733125,
    "p95": 0.454240625,
    "p99": 0.454240625,
    "p99.9": 0.454240625,
    "p99.99": 0.454240625
  },
  "responseTimeHistogram": {
    "0.025575416": 1,
    "0.0684419369": 9,
    "0.1113084578": 0,
    "0.1541749787": 0,
    "0.19704149959999998": 0,
    "0.23990802049999996": 0,
    "0.2827745414": 0,
    "0.3256410623": 0,
    "0.3685075832": 0,
    "0.41137410409999997": 0,
    "0.45424062499999995": 2
  },
  "rps": {
    "max": 1091703.0568139108,
    "mean": 91034.26228249371,
    "percentiles": {
      "p10": 17.292212452122193,
      "p25": 65.95870984763522,
      "p50": 67.7040415318256,
      "p75": 94.76839599087975,
      "p90": 111.1811057939359,
      "p95": 1091703.0568139108,
      "p99": 1091703.0568139108,
      "p99.9": 1091703.0568139108,
      "p99.99": 1091703.0568139108
    },
    "stddev": 315128.94569284806
  },
  "statusCodeDistribution": {
    "200": 12
  },
  "summary": {
    "average": 0.10357136808333334,
    "fastest": 0.025575416,
    "requestsPerSec": 13.962799611136033,
    "sizePerRequest": 547,
    "sizePerSec": 6547.5556747934315,
    "slowest": 0.454240625,
    "successRate": 1.0,
    "total": 1.00266425,
    "totalData": 6565
  }
}
```

```json
{
  "details": {
    "DNSDialup": {
      "average": 0.0,
      "fastest": null,
      "slowest": null
    },
    "DNSLookup": {
      "average": 0.0,
      "fastest": null,
      "slowest": null
    }
  },
  "errorDistribution": {
    "aborted due to deadline": 2
  },
  "latencyPercentiles": {
    "p10": null,
    "p25": null,
    "p50": null,
    "p75": null,
    "p90": null,
    "p95": null,
    "p99": null,
    "p99.9": null,
    "p99.99": null
  },
  "responseTimeHistogram": {
    "NaN": 0
  },
  "rps": {
    "max": null,
    "mean": 0.0,
    "percentiles": {
      "p10": null,
      "p25": null,
      "p50": null,
      "p75": null,
      "p90": null,
      "p95": null,
      "p99": null,
      "p99.9": null,
      "p99.99": null
    },
    "stddev": 0.0
  },
  "statusCodeDistribution": {},
  "summary": {
    "average": null,
    "fastest": null,
    "requestsPerSec": 1.9951993826885033,
    "sizePerRequest": null,
    "sizePerSec": 0.0,
    "slowest": null,
    "successRate": null,
    "total": 1.002406084,
    "totalData": 0
  }
}
```