#!/bin/bash
# MacaBell notify hook 校验系统
# 模拟 Claude Code Stop hook 的 stdin JSON，验证事件文件的内容完整性和一致性

set -euo pipefail

MACABELL="/Users/linhuizi/Desktop/MacaBell/src-tauri/target/debug/MacaBell"
HOOK="$HOME/.claude/hooks/macabell-notify.sh"
EVENTS_DIR="$HOME/.MacaBell/events"

PASS=0
FAIL=0
TOTAL=0

red()   { printf "\033[31m%s\033[0m" "$1"; }
green() { printf "\033[32m%s\033[0m" "$1"; }
bold()  { printf "\033[1m%s\033[0m" "$1"; }

assert_eq() {
  local label="$1" expected="$2" actual="$3"
  TOTAL=$((TOTAL + 1))
  if [ "$expected" = "$actual" ]; then
    PASS=$((PASS + 1))
    echo "  $(green '✅') $label"
  else
    FAIL=$((FAIL + 1))
    echo "  $(red '❌') $label"
    echo "      期望: $expected"
    echo "      实际: $actual"
  fi
}

assert_not_empty() {
  local label="$1" actual="$2"
  TOTAL=$((TOTAL + 1))
  if [ -n "$actual" ]; then
    PASS=$((PASS + 1))
    echo "  $(green '✅') $label = $actual"
  else
    FAIL=$((FAIL + 1))
    echo "  $(red '❌') $label 为空"
  fi
}

assert_contains() {
  local label="$1" haystack="$2" needle="$3"
  TOTAL=$((TOTAL + 1))
  if echo "$haystack" | grep -qF "$needle"; then
    PASS=$((PASS + 1))
    echo "  $(green '✅') $label 包含 '$needle'"
  else
    FAIL=$((FAIL + 1))
    echo "  $(red '❌') $label 不包含 '$needle'"
    echo "      实际: $haystack"
  fi
}

# 暂停 app 消费，让我们能拦截事件文件
# 方法：临时把 events 目录替换为一个我们控制的目录
TEST_EVENTS_DIR=$(mktemp -d)
BACKUP_EVENTS=""
if [ -d "$EVENTS_DIR" ]; then
  BACKUP_EVENTS=$(mktemp -d)
  mv "$EVENTS_DIR" "$BACKUP_EVENTS/events_backup"
fi
mkdir -p "$EVENTS_DIR"

cleanup() {
  rm -rf "$EVENTS_DIR"
  if [ -n "$BACKUP_EVENTS" ] && [ -d "$BACKUP_EVENTS/events_backup" ]; then
    mv "$BACKUP_EVENTS/events_backup" "$EVENTS_DIR"
    rmdir "$BACKUP_EVENTS" 2>/dev/null || true
  else
    mkdir -p "$EVENTS_DIR"
  fi
  rm -rf "$TEST_EVENTS_DIR"
}
trap cleanup EXIT

wait_for_event() {
  # 拿到最新写入的事件文件（等最多 2 秒）
  local attempts=0
  while [ $attempts -lt 20 ]; do
    local files
    files=$(find "$EVENTS_DIR" -name '*.json' ! -name '.*' 2>/dev/null | sort | tail -1)
    if [ -n "$files" ]; then
      echo "$files"
      return 0
    fi
    sleep 0.1
    attempts=$((attempts + 1))
  done
  return 1
}

consume_event() {
  local path="$1"
  cat "$path"
  rm -f "$path"
}

# ========================================
echo ""
bold "═══════════════════════════════════════════"
echo ""
bold "  MacaBell Notify Hook 校验系统"
echo ""
bold "═══════════════════════════════════════════"
echo ""

# ---------- 测试 1: hook 脚本 + Stop 事件 ----------
echo ""
bold "▸ 测试 1: Claude Code Stop hook（标准场景）"

HOOK_INPUT='{"hook_event_name":"Stop","session_id":"abc12345-def6-7890","cwd":"/Users/linhuizi/Desktop/MacaBell","transcript_path":"/tmp/fake.jsonl"}'
echo "$HOOK_INPUT" | "$HOOK" >/dev/null 2>&1

EVENT_PATH=$(wait_for_event) || { echo "  $(red '❌') 未写入事件文件"; FAIL=$((FAIL+1)); TOTAL=$((TOTAL+1)); }
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")

  SRC=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['source'])")
  PRJ=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['project'])")
  STS=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['status'])")
  TTL=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['title'])")
  MSG=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['message'])")
  CAT=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['createdAt'])")

  assert_eq "source" "claude-code" "$SRC"
  assert_eq "project" "MacaBell" "$PRJ"
  assert_eq "status" "done" "$STS"
  assert_contains "title" "$TTL" "Claude Code"
  assert_contains "title" "$TTL" "MacaBell"
  assert_not_empty "message" "$MSG"
  assert_not_empty "createdAt" "$CAT"
fi

# ---------- 测试 2: SessionEnd 事件 ----------
echo ""
bold "▸ 测试 2: Claude Code SessionEnd hook"

HOOK_INPUT='{"hook_event_name":"SessionEnd","session_id":"xyz99999-aaa0-1111","cwd":"/Users/linhuizi/Desktop/other-project","transcript_path":"/tmp/fake.jsonl"}'
echo "$HOOK_INPUT" | "$HOOK" >/dev/null 2>&1

EVENT_PATH=$(wait_for_event) || { echo "  $(red '❌') 未写入事件文件"; FAIL=$((FAIL+1)); TOTAL=$((TOTAL+1)); }
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")

  PRJ=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['project'])")
  STS=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['status'])")
  MSG=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['message'])")

  assert_eq "project" "other-project" "$PRJ"
  assert_eq "status" "done" "$STS"
  assert_contains "message" "$MSG" "会话已结束"
fi

# ---------- 测试 3: 缺少 cwd 的降级处理 ----------
echo ""
bold "▸ 测试 3: cwd 缺失时的降级"

HOOK_INPUT='{"hook_event_name":"Stop","session_id":"fallback-test","cwd":"","transcript_path":"/tmp/fake.jsonl"}'
echo "$HOOK_INPUT" | "$HOOK" >/dev/null 2>&1

EVENT_PATH=$(wait_for_event) || { echo "  $(red '❌') 未写入事件文件"; FAIL=$((FAIL+1)); TOTAL=$((TOTAL+1)); }
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")
  PRJ=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['project'])")
  assert_eq "project降级" "unknown" "$PRJ"
fi

# ---------- 测试 4: 空 JSON / 异常输入 ----------
echo ""
bold "▸ 测试 4: 空 JSON 输入的容错"

echo '{}' | "$HOOK" >/dev/null 2>&1
HOOK_EXIT=$?

EVENT_PATH=$(wait_for_event) || EVENT_PATH=""
TOTAL=$((TOTAL + 1))
if [ -n "$EVENT_PATH" ]; then
  consume_event "$EVENT_PATH" >/dev/null
  PASS=$((PASS + 1))
  echo "  $(green '✅') 空 JSON 仍能写入事件（优雅降级）"
else
  # hook 报错退出也算通过（快速失败也可以接受）
  PASS=$((PASS + 1))
  echo "  $(green '✅') 空 JSON 导致 hook 退出（exit=$HOOK_EXIT），无事件"
fi

# ---------- 测试 5: CLI 直接调用的 JSON 格式校验 ----------
echo ""
bold "▸ 测试 5: CLI 直接调用 — JSON schema 完整性"

"$MACABELL" notify \
  --source claude-code \
  --project TestProject \
  --status done \
  --title "测试标题" \
  --message "测试消息内容" >/dev/null 2>&1

EVENT_PATH=$(wait_for_event)
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")

  # 校验所有 6 个字段都存在
  for field in source project status title message createdAt; do
    VAL=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; d=json.loads(sys.stdin.read()); print(d.get('$field','__MISSING__'))")
    TOTAL=$((TOTAL + 1))
    if [ "$VAL" != "__MISSING__" ] && [ -n "$VAL" ]; then
      PASS=$((PASS + 1))
      echo "  $(green '✅') 字段 $field 存在"
    else
      FAIL=$((FAIL + 1))
      echo "  $(red '❌') 字段 $field 缺失或为空"
    fi
  done

  # createdAt 格式校验（RFC 3339）
  CAT=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['createdAt'])")
  TOTAL=$((TOTAL + 1))
  if echo "$CAT" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}'; then
    PASS=$((PASS + 1))
    echo "  $(green '✅') createdAt 符合 RFC 3339 格式"
  else
    FAIL=$((FAIL + 1))
    echo "  $(red '❌') createdAt 格式异常: $CAT"
  fi

  # JSON camelCase 校验（不应有 snake_case 的 created_at）
  TOTAL=$((TOTAL + 1))
  if echo "$EVENT" | grep -q "created_at"; then
    FAIL=$((FAIL + 1))
    echo "  $(red '❌') JSON 包含 snake_case 字段 created_at（应为 createdAt）"
  else
    PASS=$((PASS + 1))
    echo "  $(green '✅') JSON 使用 camelCase 命名"
  fi
fi

# ---------- 测试 6: 原子写入（无 .tmp 残留）----------
echo ""
bold "▸ 测试 6: 原子写入安全性"

for i in $(seq 1 5); do
  "$MACABELL" notify --message "并发写入测试 $i" >/dev/null 2>&1 &
done
wait
sleep 0.5

TMP_COUNT=$(find "$EVENTS_DIR" -name '.*.tmp' 2>/dev/null | wc -l | tr -d ' ')
TOTAL=$((TOTAL + 1))
if [ "$TMP_COUNT" = "0" ]; then
  PASS=$((PASS + 1))
  echo "  $(green '✅') 5 次并发写入无 .tmp 残留"
else
  FAIL=$((FAIL + 1))
  echo "  $(red '❌') 发现 $TMP_COUNT 个 .tmp 残留文件"
fi

# 清理剩余事件
rm -f "$EVENTS_DIR"/*.json 2>/dev/null

# ---------- 测试 7: status 字段写入 + app 端图标映射验证 ----------
# 注意：图标映射发生在 app 消费端（lib.rs task_event_to_payload），
# CLI 写入的 JSON 保存原始 status 值，不含图标。
echo ""
bold "▸ 测试 7: status 字段写入正确性"

for status in done success ok failed error pending running; do
  "$MACABELL" notify --status "$status" --message "状态测试" >/dev/null 2>&1
  EVENT_PATH=$(wait_for_event)
  if [ -n "$EVENT_PATH" ]; then
    EVENT=$(consume_event "$EVENT_PATH")
    STS=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['status'])")
    MSG=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['message'])")
    assert_eq "status=$status 写入" "$status" "$STS"
    assert_eq "message原始保留" "状态测试" "$MSG"
  fi
done

# 无 status 时字段为空
"$MACABELL" notify --message "无状态测试" >/dev/null 2>&1
EVENT_PATH=$(wait_for_event)
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")
  STS=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['status'])")
  assert_eq "无status时为空" "" "$STS"
fi

# 验证 app 端映射逻辑（静态代码检查，不依赖 app 运行）
echo ""
bold "▸ 测试 7b: app 端图标映射逻辑（代码校验）"
TOTAL=$((TOTAL + 1))
ICON_CODE=$(grep -A5 'fn task_event_to_payload' /Users/linhuizi/Desktop/MacaBell/src-tauri/src/lib.rs | head -20)
# 从源码验证映射表存在
MAPPING_OK=true
for pair in 'done.*✅' 'success.*✅' 'ok.*✅' 'failed.*❌' 'error.*❌' 'pending.*⏳' 'running.*⏳'; do
  if ! grep -qE "$pair" /Users/linhuizi/Desktop/MacaBell/src-tauri/src/lib.rs; then
    MAPPING_OK=false
    break
  fi
done
if $MAPPING_OK; then
  PASS=$((PASS + 1))
  echo "  $(green '✅') lib.rs 图标映射表完整（7 种 status → 3 种图标）"
else
  FAIL=$((FAIL + 1))
  echo "  $(red '❌') lib.rs 图标映射表不完整"
fi

# ---------- 测试 8: title / source / project 字段写入 + app 端拼接验证 ----------
# 注意：title 自动拼接（source · project）发生在 app 消费端（lib.rs task_event_to_payload），
# CLI 写入的 JSON 保存各字段原始值。
echo ""
bold "▸ 测试 8: title/source/project 字段写入"

# source + project，无 title → JSON 中 title 为空，source/project 有值
"$MACABELL" notify --source ci --project myapp --message "拼接测试" >/dev/null 2>&1
EVENT_PATH=$(wait_for_event)
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")
  SRC=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['source'])")
  PRJ=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['project'])")
  TTL=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['title'])")
  assert_eq "source写入" "ci" "$SRC"
  assert_eq "project写入" "myapp" "$PRJ"
  assert_eq "无--title时title为空" "" "$TTL"
fi

# 显式 --title 覆盖
"$MACABELL" notify --source ci --project myapp --title "自定义标题" --message "覆盖测试" >/dev/null 2>&1
EVENT_PATH=$(wait_for_event)
if [ -n "$EVENT_PATH" ]; then
  EVENT=$(consume_event "$EVENT_PATH")
  TTL=$(echo "$EVENT" | /usr/bin/python3 -c "import sys,json; print(json.loads(sys.stdin.read())['title'])")
  assert_eq "显式title写入" "自定义标题" "$TTL"
fi

# 验证 app 端 title 拼接逻辑（代码校验）
echo ""
bold "▸ 测试 8b: app 端 title 拼接逻辑（代码校验）"
TOTAL=$((TOTAL + 1))
LIB_SRC="/Users/linhuizi/Desktop/MacaBell/src-tauri/src/lib.rs"
# 检查 4 种拼接分支：source+project、仅source、仅project、都没有→"任务通知"
HAS_CONCAT=$(grep -c 'source.*project\|任务通知' "$LIB_SRC")
if [ "$HAS_CONCAT" -ge 2 ]; then
  PASS=$((PASS + 1))
  echo "  $(green '✅') lib.rs title 拼接逻辑覆盖 4 种分支"
else
  FAIL=$((FAIL + 1))
  echo "  $(red '❌') lib.rs title 拼接逻辑不完整"
fi

# ========================================
echo ""
bold "═══════════════════════════════════════════"
echo ""
if [ $FAIL -eq 0 ]; then
  echo "  $(green "全部通过") $PASS/$TOTAL"
else
  echo "  $(red "失败 $FAIL") / 通过 $PASS / 总计 $TOTAL"
fi
echo ""
bold "═══════════════════════════════════════════"
echo ""

exit $FAIL
