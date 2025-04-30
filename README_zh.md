# LongTime

多时区时间管理工具

一个基于 Rust 和 ratatui 库开发的命令行终端用户界面 (TUI) 工具，用于管理和显示多个时区的时间信息。

## 功能特点

- **多时区支持**：同时显示多个时区的当前时间
- **工作时间显示**：为每个时区显示配置的工作时间范围
- **工作状态标识**：直观地显示每个时区是否处于工作时间内
- **时间调整**：可手动调整任意时区的时间，其他时区时间会同步更新
- **配置文件支持**：通过 TOML 配置文件轻松添加和管理时区
- **交互式界面**：使用键盘导航和操作的用户友好界面

## 快速开始

### 安装

1. 编译项目：

```bash
cargo build --release
```

2. 运行程序：

```bash
cargo run --release
```

或者直接运行编译好的二进制文件：

```bash
./target/release/time
```

### 配置

程序使用 `timezones.toml` 配置文件来定义时区信息。该文件应放在程序执行的当前目录下。

配置文件示例：

```toml
# 多时区配置文件

[[timezones]]
name = "Beijing"
timezone = "Asia/Shanghai"
work_hours = { start = "09:00", end = "18:00" }

[[timezones]]
name = "New_York"
timezone = "America/New_York"
work_hours = { start = "09:00", end = "17:00" }

[[timezones]]
name = "London"
timezone = "Europe/London"
work_hours = { start = "09:00", end = "17:30" }

[[timezones]]
name = "Tokyo"
timezone = "Asia/Tokyo"
work_hours = { start = "09:30", end = "18:30" }

[[timezones]]
name = "Sydney"
timezone = "Australia/Sydney"
work_hours = { start = "09:00", end = "17:00" }
```

配置项说明：
- `name`：时区显示名称
- `timezone`：时区标识符（符合 IANA 时区数据库格式）
- `work_hours`：工作时间范围，包含 `start`（开始时间）和 `end`（结束时间）

## 使用方法

### 界面导航

程序启动后，你会看到一个包含所有配置的时区信息的列表。每个时区条目显示：
- 时区名称
- 当前时间（格式：YYYY-MM-DD HH:MM:SS）
- 工作时间范围
- 当前工作状态（工作时间/非工作时间）

### 键盘快捷键

| 键位 | 功能 |
|------|------|
| `↑` (上箭头) | 选择上一个时区 |
| `↓` (下箭头) | 选择下一个时区 |
| `←` (左箭头) | 将时间调整回退 30 分钟 |
| `→` (右箭头) | 将时间调整前进 30 分钟 |
| `q` | 退出程序 |

### 时间调整功能

当你使用左右箭头键调整时间时，所有时区的时间会同步更新。这个功能允许你：

1. 查看不同时区在特定时间点的状态
2. 规划跨时区的会议或活动
3. 估算不同时区的工作时间重叠情况

## 自定义和扩展

### 添加新时区

要添加新的时区，只需在 `timezones.toml` 文件中添加新的 `[[timezones]]` 条目：

```toml
[[timezones]]
name = "Singapore"
timezone = "Asia/Singapore"
work_hours = { start = "09:00", end = "18:00" }
```

### 修改工作时间

要修改现有时区的工作时间，更新对应时区条目的 `work_hours` 值：

```toml
[[timezones]]
name = "London"
timezone = "Europe/London"
work_hours = { start = "08:30", end = "16:30" }  # 更新后的工作时间
```

### 支持的时区格式

本工具使用 `chrono-tz` 库，支持 IANA 时区数据库中的所有时区标识符。常见的时区标识符包括：

- `Asia/Shanghai`（中国北京时间）
- `America/New_York`（美国东部时间）
- `Europe/London`（英国伦敦时间）
- `Asia/Tokyo`（日本东京时间）
- `Australia/Sydney`（澳大利亚悉尼时间）
- `Europe/Paris`（法国巴黎时间）

完整的时区列表可在[这里](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)查看。

## 技术架构

本工具使用以下 Rust 库：

- **ratatui**：用于创建终端用户界面
- **crossterm**：用于处理终端事件和控制
- **chrono** 和 **chrono-tz**：用于时区和时间计算
- **serde** 和 **toml**：用于配置文件解析

## 常见问题

### Q: 程序无法启动，显示配置文件错误？
A: 确保 `timezones.toml` 文件位于程序执行的当前目录中，并且符合正确的 TOML 语法格式。

### Q: 如何恢复到真实的当前时间？
A: 退出程序并重新启动可以重置时间偏移。

### Q: 可以更改时间调整的步长吗？
A: 目前时间调整的步长固定为 30 分钟。如需修改，可以编辑源代码中的 `adjust_time_forward` 和 `adjust_time_backward` 函数。
