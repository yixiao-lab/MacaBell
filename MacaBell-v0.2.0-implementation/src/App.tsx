import { useEffect, useState, type CSSProperties } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface ReminderPayload {
  id?: string;
  title: string;
  message: string;
}

interface QueuedReminder extends Required<ReminderPayload> {
  instanceId: string;
  palette: Palette;
  visible: boolean;
}

// 马卡龙配色：每套含 背景渐变 / 标题色 / 正文色 / 圆点渐变 / 阴影色
interface Palette {
  bg: string;
  title: string;
  text: string;
  dot: string;
  shadow: string;
}

const PALETTES: Palette[] = [
  // 草莓粉
  {
    bg: "linear-gradient(135deg, #fff7fb 0%, #ffd6e8 100%)",
    title: "#d65a93",
    text: "#6b5b73",
    dot: "linear-gradient(135deg, #ff9ec4, #c8f4e3)",
    shadow: "rgba(255, 158, 196, 0.35)",
  },
  // 薄荷绿
  {
    bg: "linear-gradient(135deg, #f3fffb 0%, #c8f4e3 100%)",
    title: "#3fa98a",
    text: "#5a6b66",
    dot: "linear-gradient(135deg, #7ad6b8, #d6f0ff)",
    shadow: "rgba(120, 214, 184, 0.35)",
  },
  // 薰衣草紫
  {
    bg: "linear-gradient(135deg, #fbf7ff 0%, #e3d6ff 100%)",
    title: "#8b6fc4",
    text: "#645b73",
    dot: "linear-gradient(135deg, #b89be8, #ffd6e8)",
    shadow: "rgba(184, 155, 232, 0.35)",
  },
  // 柠檬黄
  {
    bg: "linear-gradient(135deg, #fffdf3 0%, #fff3c8 100%)",
    title: "#c9a23f",
    text: "#6b6552",
    dot: "linear-gradient(135deg, #f5d678, #fff0c8)",
    shadow: "rgba(245, 214, 120, 0.4)",
  },
  // 天空蓝
  {
    bg: "linear-gradient(135deg, #f3fbff 0%, #cfe8ff 100%)",
    title: "#4f8fc4",
    text: "#5b6673",
    dot: "linear-gradient(135deg, #8cc4f0, #d6f0ff)",
    shadow: "rgba(140, 196, 240, 0.35)",
  },
  // 蜜桃橙
  {
    bg: "linear-gradient(135deg, #fff7f3 0%, #ffe0cf 100%)",
    title: "#d6824f",
    text: "#735f52",
    dot: "linear-gradient(135deg, #f5a878, #ffe0cf)",
    shadow: "rgba(245, 168, 120, 0.35)",
  },
];

function randomPalette(): Palette {
  return PALETTES[Math.floor(Math.random() * PALETTES.length)];
}

function createQueuedReminder(payload: ReminderPayload): QueuedReminder {
  const id = payload.id ?? "reminder";

  return {
    id,
    title: payload.title,
    message: payload.message,
    instanceId: `${id}-${Date.now()}-${Math.random().toString(36).slice(2)}`,
    palette: randomPalette(),
    visible: false,
  };
}

function createStyleVars(palette: Palette, index: number): CSSProperties {
  return {
    "--card-bg": palette.bg,
    "--card-title": palette.title,
    "--card-text": palette.text,
    "--card-dot": palette.dot,
    "--card-shadow": palette.shadow,
    "--card-delay": `${index * 24}ms`,
  } as CSSProperties;
}

function App() {
  const [reminders, setReminders] = useState<QueuedReminder[]>([]);

  useEffect(() => {
    const unlisten = listen<ReminderPayload>("reminder", (event) => {
      const item = createQueuedReminder(event.payload);

      setReminders((prev) => [...prev, item]);

      // 下一帧再触发滑入，保证动画从初始态开始
      requestAnimationFrame(() => {
        setReminders((prev) =>
          prev.map((current) =>
            current.instanceId === item.instanceId
              ? { ...current, visible: true }
              : current,
          ),
        );
      });
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  function close(instanceId: string) {
    setReminders((prev) =>
      prev.map((item) =>
        item.instanceId === instanceId ? { ...item, visible: false } : item,
      ),
    );

    // 等滑出动画结束再从队列移除。最后一条关闭后，隐藏透明窗口。
    setTimeout(() => {
      setReminders((prev) => {
        const next = prev.filter((item) => item.instanceId !== instanceId);

        if (next.length === 0) {
          invoke("hide_popup");
        }

        return next;
      });
    }, 320);
  }

  if (reminders.length === 0) return null;

  return (
    <div className="stack">
      {reminders.map((reminder, index) => (
        <div
          key={reminder.instanceId}
          className={`card ${reminder.visible ? "card--in" : ""}`}
          style={createStyleVars(reminder.palette, index)}
        >
          <div className="card__dot" />

          <div className="card__body">
            <div className="card__title">{reminder.title}</div>
            <div className="card__message">{reminder.message}</div>
          </div>

          <button
            className="card__close"
            onClick={() => close(reminder.instanceId)}
            aria-label="关闭"
          >
            ×
          </button>
        </div>
      ))}
    </div>
  );
}

export default App;
