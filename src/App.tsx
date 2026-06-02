import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface ReminderPayload {
  title: string;
  message: string;
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

function App() {
  const [reminder, setReminder] = useState<ReminderPayload | null>(null);
  const [palette, setPalette] = useState<Palette>(PALETTES[0]);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const unlisten = listen<ReminderPayload>("reminder", (event) => {
      setPalette(randomPalette());
      setReminder(event.payload);
      // 下一帧再触发滑入，保证动画从初始态开始
      requestAnimationFrame(() => setVisible(true));
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  async function close() {
    setVisible(false);
    // 等滑出动画结束再隐藏窗口
    setTimeout(() => {
      invoke("hide_popup");
      setReminder(null);
    }, 320);
  }

  if (!reminder) return null;

  const styleVars = {
    "--card-bg": palette.bg,
    "--card-title": palette.title,
    "--card-text": palette.text,
    "--card-dot": palette.dot,
    "--card-shadow": palette.shadow,
  } as React.CSSProperties;

  return (
    <div className={`card ${visible ? "card--in" : ""}`} style={styleVars}>
      <div className="card__dot" />
      <div className="card__body">
        <div className="card__title">{reminder.title}</div>
        <div className="card__message">{reminder.message}</div>
      </div>
      <button className="card__close" onClick={close} aria-label="关闭">
        ×
      </button>
    </div>
  );
}

export default App;
