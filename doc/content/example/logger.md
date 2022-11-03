---
title: "Logger"
date: 2022-09-23T15:05:28Z
weight: 2
---

### コマンド

```text
./sysdc parse logger.def string.def io.def time.def
```

### logger.def

```text
unit logger;

from string import String;
from io import IO;
from time import Time, Timestamp;

module Logger {
    proc info(msg: String) {
        @affect IO.stdout(msg)

        @modify msg {
            use timestamp;
        }

        @spawn timestamp: Timestamp {
            let now = Time.get_now_time();
            return now;
        }
    }

    proc error(msg: String) {
        @affect IO.stdout(msg)

        @modify msg {
            use timestamp;
        }

        @spawn timestamp: Timestamp {
            let now = Time.get_now_time();
            return now;
        }
    }
}
```

### string.def

```text
unit string;

data String {}
```

### io.def

```text
unit io;

module IO {
    proc stdout(msg: String) {}
}
```

### time.def

```text
unit time;

data Timestamp {}

module Time {
    func get_now_time() -> Timestamp {
        @return now
        @spawn now: Timestamp
    }
}
```
