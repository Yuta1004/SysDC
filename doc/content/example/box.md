---
title: "Box"
date: 2022-09-23T15:02:47Z
weight: 1
---

### コマンド

```text
./sysdc parse box.def
```

### box.def

```text
unit box;

data Box {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

module BoxModule {
    func new(x: i32, y: i32, w: i32, h: i32) -> Box {
        @return box

        @spawn box: Box {
            use x, y, w, h;
        }
    }

    proc move(box: Box, dx: i32, dy: i32) {
        @modify box {
            use dx, dy;
        }
    }

    proc change_size(box: Box, w: i32, h: i32) {
        @modify box {
            use w, h;
        }
    }
}
```
