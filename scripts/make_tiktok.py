import asyncio
import math
import os
import subprocess
import time
import cv2
import edge_tts
import imageio_ffmpeg
import numpy as np
from PIL import Image, ImageDraw, ImageFilter, ImageFont

W, H = 1080, 1920
FPS = 60

REPO = r"c:\Users\jorge\Desktop\Proyectos\AirDrop Bridge"
OUT_DIR = os.path.join(REPO, "output")
SCRATCH_DIR = r"C:\Users\jorge\.gemini\antigravity\brain\d1eff2b4-f67d-4c55-978a-5e0f637cb213\scratch\tiktok"
os.makedirs(OUT_DIR, exist_ok=True)
os.makedirs(SCRATCH_DIR, exist_ok=True)

FFMPEG = imageio_ffmpeg.get_ffmpeg_exe()

FONT_BLACK = r"C:\Windows\Fonts\seguibl.ttf"
FONT_BOLD = r"C:\Windows\Fonts\segoeuib.ttf"
FONT_SEMI = r"C:\Windows\Fonts\seguisb.ttf"
FONT_REG = r"C:\Windows\Fonts\segoeui.ttf"
FONT_MONO = r"C:\Windows\Fonts\consolab.ttf"

_font_cache = {}
def font(path, size):
    key = (path, size)
    if key not in _font_cache:
        _font_cache[key] = ImageFont.truetype(path, size)
    return _font_cache[key]

def clamp01(x):
    return 0.0 if x < 0.0 else (1.0 if x > 1.0 else x)

def ease_out_cubic(t):
    t = clamp01(t)
    return 1.0 - (1.0 - t) ** 3

def ease_in_out_cubic(t):
    t = clamp01(t)
    return 4 * t * t * t if t < 0.5 else 1 - ((-2 * t + 2) ** 3) / 2

def ease_out_back(t, s=1.45):
    t = clamp01(t)
    t -= 1.0
    return t * t * ((s + 1.0) * t + s) + 1.0

def lerp(a, b, t):
    return a + (b - a) * t

def build_base_bg():
    y = np.linspace(0, 1, H, dtype=np.float32)[:, None]
    r = (8 + 10 * np.sin(y * math.pi)).astype(np.uint8)
    g = (12 + 14 * np.sin(y * math.pi)).astype(np.uint8)
    b = (22 + 20 * np.sin(y * math.pi)).astype(np.uint8)
    arr = np.zeros((H, W, 3), dtype=np.uint8)
    arr[:, :, 0] = r
    arr[:, :, 1] = g
    arr[:, :, 2] = b
    # Subtle dot grid
    arr[24::48, 24::48, 0] = np.minimum(255, arr[24::48, 24::48, 0].astype(np.int32) + 12).astype(np.uint8)
    arr[24::48, 24::48, 1] = np.minimum(255, arr[24::48, 24::48, 1].astype(np.int32) + 16).astype(np.uint8)
    arr[24::48, 24::48, 2] = np.minimum(255, arr[24::48, 24::48, 2].astype(np.int32) + 24).astype(np.uint8)
    return arr

def make_glow(diameter, color_rgb, max_alpha=0.25):
    s = 256
    yy, xx = np.mgrid[0:s, 0:s]
    dist = np.sqrt(((xx - s / 2) / (s / 2)) ** 2 + ((yy - s / 2) / (s / 2)) ** 2)
    falloff = np.clip(1.0 - dist, 0.0, 1.0) ** 2.2
    rgba = np.zeros((s, s, 4), dtype=np.uint8)
    rgba[:, :, 0] = color_rgb[0]
    rgba[:, :, 1] = color_rgb[1]
    rgba[:, :, 2] = color_rgb[2]
    rgba[:, :, 3] = (falloff * 255 * max_alpha).astype(np.uint8)
    return Image.fromarray(rgba, "RGBA").resize((diameter, diameter), Image.Resampling.BILINEAR)

BASE_BG = build_base_bg()
GLOW_CYAN = make_glow(1000, (56, 189, 248), 0.24)
GLOW_INDIGO = make_glow(950, (99, 102, 241), 0.20)
GLOW_ORANGE = make_glow(900, (249, 115, 22), 0.18)

def safe_composite(dst, src, x, y):
    sw, sh = src.size
    dw, dh = dst.size
    x0 = max(0, x)
    y0 = max(0, y)
    x1 = min(dw, x + sw)
    y1 = min(dh, y + sh)
    if x1 <= x0 or y1 <= y0:
        return
    sx0 = x0 - x
    sy0 = y0 - y
    sx1 = sx0 + (x1 - x0)
    sy1 = sy0 + (y1 - y0)
    dst.alpha_composite(src, (x0, y0), (sx0, sy0, sx1, sy1))

def draw_centered_text(draw, y, text, fnt, fill, shadow=True):
    a = fill[3] if len(fill) == 4 else 255
    if a <= 2:
        return 0
    bbox = draw.textbbox((0, 0), text, font=fnt)
    tw = bbox[2] - bbox[0]
    x = (W - tw) // 2
    if shadow:
        sh_a = int(160 * (a / 255.0))
        draw.text((x, y + 4), text, font=fnt, fill=(0, 0, 0, sh_a))
    draw.text((x, y), text, font=fnt, fill=fill)
    return bbox[3] - bbox[1]

def draw_pill_badge(img, cx, cy, text, fnt, bg=(18, 26, 44, 230), border=(56, 189, 248, 140), fg=(240, 248, 255, 255), pad_x=32, pad_y=16, alpha=1.0):
    if alpha <= 0.01:
        return
    tmp = Image.new("RGBA", (W, 140), (0, 0, 0, 0))
    d = ImageDraw.Draw(tmp, "RGBA")
    bbox = d.textbbox((0, 0), text, font=fnt)
    tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
    bw, bh = tw + pad_x * 2, th + pad_y * 2
    x0 = cx - bw // 2
    y0 = 70 - bh // 2
    bg_a = (*bg[:3], int(bg[3] * alpha))
    bd_a = (*border[:3], int(border[3] * alpha))
    fg_a = (*fg[:3], int(fg[3] * alpha))
    d.rounded_rectangle([x0, y0, x0 + bw, y0 + bh], radius=bh // 2, fill=bg_a, outline=bd_a, width=2)
    d.text((x0 + pad_x, y0 + pad_y - bbox[1]), text, font=fnt, fill=fg_a)
    safe_composite(img, tmp, 0, int(cy - 70))

def paste_centered(canvas, img, cx, cy, scale=1.0, alpha=1.0):
    if alpha <= 0.01 or scale <= 0.01:
        return
    if abs(scale - 1.0) > 0.004:
        nw = max(2, int(round(img.width * scale)))
        nh = max(2, int(round(img.height * scale)))
        im = img.resize((nw, nh), Image.Resampling.BILINEAR)
    else:
        im = img
    if alpha < 0.995:
        im = im.copy()
        a = im.split()[3].point(lambda v: int(v * alpha))
        im.putalpha(a)
    x0 = int(round(cx - im.width / 2))
    y0 = int(round(cy - im.height / 2))
    safe_composite(canvas, im, x0, y0)

# Pre-render Assets
icon_path = os.path.join(REPO, "resources", "icon.png")
icon_raw = Image.open(icon_path).convert("RGBA").resize((260, 260), Image.Resampling.LANCZOS)
_imask = Image.new("L", (260, 260), 0)
ImageDraw.Draw(_imask).rounded_rectangle([0, 0, 259, 259], radius=58, fill=255)
LOGO_ROUNDED = Image.new("RGBA", (260, 260), (0, 0, 0, 0))
LOGO_ROUNDED.paste(icon_raw, (0, 0), _imask)

# Add glow + drop shadow around logo
_lcanvas = Image.new("RGBA", (340, 340), (0, 0, 0, 0))
_lsh = Image.new("RGBA", (340, 340), (0, 0, 0, 0))
ImageDraw.Draw(_lsh).rounded_rectangle([40, 48, 300, 308], radius=58, fill=(0, 0, 0, 180))
_lsh = _lsh.filter(ImageFilter.GaussianBlur(24))
_lcanvas.alpha_composite(_lsh)
_lcanvas.alpha_composite(LOGO_ROUNDED, (40, 40))
_ld = ImageDraw.Draw(_lcanvas, "RGBA")
_ld.rounded_rectangle([40, 40, 299, 299], radius=58, outline=(56, 189, 248, 160), width=3)
LOGO_PREPARED = _lcanvas

# iPhone Mockup
def build_iphone_mockup():
    share_path = os.path.join(REPO, "assets", "screenshots", "ios-share-sheet.png")
    ss = Image.open(share_path).convert("RGBA")
    s_w, s_h = 490, 1060
    ss = ss.resize((s_w, s_h), Image.Resampling.LANCZOS)
    
    bezel = 16
    p_w = s_w + bezel * 2
    p_h = s_h + bezel * 2
    corner_r = 52
    
    phone = Image.new("RGBA", (p_w, p_h), (0, 0, 0, 0))
    pd = ImageDraw.Draw(phone)
    pd.rounded_rectangle([0, 0, p_w, p_h], radius=corner_r, fill=(24, 27, 34, 255), outline=(71, 85, 105, 240), width=3)
    
    s_mask = Image.new("L", (s_w, s_h), 0)
    ImageDraw.Draw(s_mask).rounded_rectangle([0, 0, s_w, s_h], radius=38, fill=255)
    phone.paste(ss, (bezel, bezel), s_mask)
    
    # Dynamic island
    di_w, di_h = 130, 34
    di_x = (p_w - di_w) // 2
    pd.rounded_rectangle([di_x, bezel + 10, di_x + di_w, bezel + 10 + di_h], radius=17, fill=(0, 0, 0, 255))
    
    # Canvas with drop shadow
    pad = 50
    cw = p_w + pad * 2
    ch = p_h + pad * 2
    out = Image.new("RGBA", (cw, ch), (0, 0, 0, 0))
    sh = Image.new("RGBA", (cw, ch), (0, 0, 0, 0))
    ImageDraw.Draw(sh).rounded_rectangle([pad, pad + 18, pad + p_w, pad + p_h + 18], radius=corner_r, fill=(0, 0, 0, 180))
    sh = sh.filter(ImageFilter.GaussianBlur(30))
    out.alpha_composite(sh)
    out.alpha_composite(phone, (pad, pad))
    return out

IPHONE_CARD = build_iphone_mockup()

# Windows Pairing Window Mockup
def build_windows_mockup():
    pair_path = os.path.join(REPO, "assets", "screenshots", "pairing-window.png")
    pw = Image.open(pair_path).convert("RGBA")
    w_w = 640
    w_h = int(611 * (w_w / 461))  # ~848
    pw = pw.resize((w_w, w_h), Image.Resampling.LANCZOS)
    
    pad = 50
    cw = w_w + pad * 2
    ch = w_h + pad * 2
    out = Image.new("RGBA", (cw, ch), (0, 0, 0, 0))
    
    sh = Image.new("RGBA", (cw, ch), (0, 0, 0, 0))
    ImageDraw.Draw(sh).rounded_rectangle([pad, pad + 18, pad + w_w, pad + w_h + 18], radius=16, fill=(0, 0, 0, 190))
    sh = sh.filter(ImageFilter.GaussianBlur(28))
    out.alpha_composite(sh)
    
    mask = Image.new("L", (w_w, w_h), 0)
    ImageDraw.Draw(mask).rounded_rectangle([0, 0, w_w, w_h], radius=16, fill=255)
    
    win = Image.new("RGBA", (w_w, w_h), (0, 0, 0, 0))
    win.paste(pw, (0, 0), mask)
    wd = ImageDraw.Draw(win, "RGBA")
    wd.rounded_rectangle([0, 0, w_w - 1, w_h - 1], radius=16, outline=(255, 255, 255, 45), width=2)
    out.alpha_composite(win, (pad, pad))
    return out

WIN_CARD = build_windows_mockup()

# Script text definitions
LINES = [
    "If you have an iPhone and a Windows PC, transferring files is a complete nightmare.",
    "So I built Drop Bridge. Real, blazing-fast file transfers from iPhone to Windows and Linux over your local Wi-Fi.",
    "And the best part? Zero apps to install on your phone. It runs through an official Apple Shortcut, straight in your native iOS Share Sheet.",
    "Under the hood, it is written in Rust with chunked streaming. Gigabyte-sized 4K videos land in your Downloads in seconds, using under 15 megabytes of RAM.",
    "Setup takes thirty seconds with a single QR code. It is completely free and open-source on GitHub. Go grab Drop Bridge today!"
]

# Scene 1: The Problem
def draw_scene_1(canvas, lt, s_dur, alpha):
    d = ImageDraw.Draw(canvas, "RGBA")
    t_head = ease_out_cubic(lt / 0.35)
    hy = int(lerp(280, 240, t_head))
    ha = int(255 * alpha * t_head)
    
    draw_centered_text(d, hy, "iPhone to Windows PC", font(FONT_BLACK, 78), (248, 250, 252, ha))
    draw_centered_text(d, hy + 90, "Transferring Files Sucks", font(FONT_BLACK, 74), (248, 113, 113, ha))
    
    bloat = [
        ("Emailing yourself", "25 MB limit & spam inbox", (239, 68, 68)),
        ("Cloud Drives (iCloud/Drive)", "Slow sync & monthly fees", (249, 115, 22)),
        ("Shady App Store Apps", "Loaded with subscriptions & ads", (234, 179, 8)),
    ]
    
    t_slash = s_dur * 0.32
    slash_p = ease_out_cubic((lt - t_slash) / 0.40)
    
    for idx, (title, sub, dot_col) in enumerate(bloat):
        ep = ease_out_back((lt - 0.15 - idx * 0.14) / 0.38)
        if ep <= 0.01:
            continue
        bw, bh = 860, 136
        bx = (W - bw) // 2
        by = int(580 + idx * 168 + (1.0 - ep) * 45)
        a_card = alpha * clamp01(ep)
        
        d.rounded_rectangle([bx, by, bx + bw, by + bh], radius=26, fill=(18, 24, 38, int(235 * a_card)), outline=(71, 85, 105, int(150 * a_card)), width=2)
        d.ellipse([bx + 36, by + 48, bx + 68, by + 80], fill=(*dot_col, int(240 * a_card)))
        d.text((bx + 94, by + 32), title, font=font(FONT_BOLD, 42), fill=(241, 245, 249, int(255 * a_card)))
        d.text((bx + 94, by + 82), sub, font=font(FONT_REG, 28), fill=(148, 163, 184, int(240 * a_card)))
        
        # Red animated slash
        if slash_p > 0.01:
            sx2 = int(lerp(bx + 24, bx + bw - 24, slash_p))
            d.line([bx + 24, by + bh // 2, sx2, by + bh // 2], fill=(239, 68, 68, int(235 * alpha)), width=7)

    t_btm = s_dur * 0.55
    bp = ease_out_cubic((lt - t_btm) / 0.35)
    if bp > 0.01:
        draw_pill_badge(canvas, W // 2, 1260, "There is a much better way...", font(FONT_BOLD, 36),
                        bg=(15, 32, 45, 240), border=(56, 189, 248, 180), fg=(56, 189, 248, 255), alpha=alpha * bp)

# Scene 2: The Solution (Drop Bridge)
def draw_scene_2(canvas, lt, s_dur, alpha):
    d = ImageDraw.Draw(canvas, "RGBA")
    t_head = ease_out_cubic(lt / 0.35)
    hy = int(lerp(270, 230, t_head))
    ha = int(255 * alpha * t_head)
    
    draw_centered_text(d, hy, "Meet Drop Bridge", font(FONT_BLACK, 84), (248, 250, 252, ha))
    draw_centered_text(d, hy + 96, "Native Local Wi-Fi Transfers", font(FONT_BLACK, 68), (56, 189, 248, ha))
    
    lp = ease_out_back((lt - 0.12) / 0.45, s=1.35)
    # Pulsing scale
    pulse = 1.0 + 0.03 * math.sin(lt * 4.0)
    cy = int(lerp(720, 680, lp))
    paste_centered(canvas, LOGO_PREPARED, W // 2, cy, scale=(0.75 + 0.25 * lp) * pulse, alpha=alpha * clamp01(lp))
    
    # Broadcast Wi-Fi waves around logo
    wave_p = ease_out_cubic((lt - 0.35) / 0.5)
    if wave_p > 0.01:
        for r_step in [190, 240, 290]:
            r = int(r_step * (0.8 + 0.2 * wave_p))
            wave_a = int(180 * (1.0 - (r_step - 190) / 130) * alpha)
            d.arc([W // 2 - r, cy - r, W // 2 + r, cy + r], start=210, end=330, fill=(56, 189, 248, wave_a), width=4)
    
    pills = [
        ("• 100% Local & Private (Direct P2P)", (16, 185, 129)),
        ("• No Cloud Storage • No Accounts", (56, 189, 248)),
        ("• Zero Subscriptions • Free Forever", (234, 179, 8)),
    ]
    
    for i, (text, col) in enumerate(pills):
        pp = ease_out_cubic((lt - 0.45 - i * 0.12) / 0.35)
        if pp <= 0.01:
            continue
        draw_pill_badge(canvas, W // 2, int(980 + i * 115), text, font(FONT_BOLD, 34),
                        bg=(18, 26, 44, 235), border=(*col, 160), fg=(241, 245, 249, 255), pad_x=38, alpha=alpha * pp)

# Scene 3: Zero Apps on iOS
def draw_scene_3(canvas, lt, s_dur, alpha):
    d = ImageDraw.Draw(canvas, "RGBA")
    t_head = ease_out_cubic(lt / 0.35)
    hy = int(lerp(240, 200, t_head))
    ha = int(255 * alpha * t_head)
    
    draw_centered_text(d, hy, "ZERO APPS ON iOS", font(FONT_BLACK, 84), (74, 222, 128, ha))
    draw_centered_text(d, hy + 94, "Official Apple Shortcut", font(FONT_BLACK, 64), (248, 250, 252, ha))
    
    phone_p = ease_out_back((lt - 0.10) / 0.48, s=1.2)
    py = int(lerp(1200, 1020, phone_p))
    paste_centered(canvas, IPHONE_CARD, W // 2, py, scale=0.92 + 0.08 * phone_p, alpha=alpha * phone_p)
    
    # Glowing pulse on "Drop Bridge" button
    tap_p = ease_out_cubic((lt - 0.45) / 0.40)
    if tap_p > 0.01:
        # Drop Bridge row in iPhone mockup coordinates
        # Center of phone is at (W // 2, py)
        # Bounding box around row
        rw, rh = 470, 68
        rx = W // 2 - rw // 2
        ry = py + 338
        
        pulse = 0.5 + 0.5 * math.sin(lt * 6.0)
        p_alpha = int(lerp(140, 255, pulse) * alpha)
        
        # Glowing border around button
        d.rounded_rectangle([rx, ry, rx + rw, ry + rh], radius=14, outline=(56, 189, 248, p_alpha), width=4)
        
        # Callout badge above button
        callout_p = ease_out_back((lt - 0.70) / 0.35)
        if callout_p > 0.01:
            draw_pill_badge(canvas, W // 2, ry - 54, "Tap Share -> Drop Bridge!", font(FONT_BOLD, 32),
                            bg=(15, 23, 42, 245), border=(56, 189, 248, 220), fg=(56, 189, 248, 255),
                            pad_x=34, pad_y=16, alpha=alpha * callout_p)

# Scene 4: Blazing Fast Rust Engine
def draw_scene_4(canvas, lt, s_dur, alpha):
    d = ImageDraw.Draw(canvas, "RGBA")
    t_head = ease_out_cubic(lt / 0.35)
    hy = int(lerp(240, 200, t_head))
    ha = int(255 * alpha * t_head)
    
    draw_centered_text(d, hy, "BLAZING FAST RUST ENGINE", font(FONT_BLACK, 74), (249, 115, 22, ha))
    draw_centered_text(d, hy + 90, "Constant Memory • < 15 MB RAM", font(FONT_BLACK, 60), (248, 250, 252, ha))
    
    win_p = ease_out_cubic((lt - 0.12) / 0.45)
    wy = int(lerp(880, 800, win_p))
    paste_centered(canvas, WIN_CARD, W // 2, wy, scale=0.90 + 0.10 * win_p, alpha=alpha * win_p)
    
    # Animated Windows 11 Notification Toast sliding in from bottom
    notif_p = ease_out_back((lt - 0.40) / 0.42, s=1.2)
    if notif_p > 0.01:
        nw, nh = 840, 154
        nx = (W - nw) // 2
        ny = int(lerp(1450, 1260, notif_p))
        na = alpha * clamp01(notif_p)
        
        # Drop shadow
        n_sh = Image.new("RGBA", (nw + 50, nh + 50), (0, 0, 0, 0))
        ImageDraw.Draw(n_sh).rounded_rectangle([25, 28, 25 + nw, 28 + nh], radius=22, fill=(0, 0, 0, 190))
        n_sh = n_sh.filter(ImageFilter.GaussianBlur(22))
        safe_composite(canvas, n_sh, nx - 25, ny - 28)
        
        d.rounded_rectangle([nx, ny, nx + nw, ny + nh], radius=22, fill=(20, 26, 42, int(245 * na)), outline=(56, 189, 248, int(180 * na)), width=2)
        
        # Toast icon
        safe_composite(canvas, LOGO_ROUNDED.resize((68, 68), Image.Resampling.LANCZOS), nx + 28, ny + 28)
        
        d.text((nx + 118, ny + 24), "Drop Bridge", font=font(FONT_BOLD, 34), fill=(241, 245, 249, int(255 * na)))
        d.text((nx + nw - 170, ny + 26), "Just now", font=font(FONT_REG, 24), fill=(148, 163, 184, int(230 * na)))
        d.text((nx + 118, ny + 68), "Photo received: IMG_4820.HEIC (12.4 MB)", font=font(FONT_SEMI, 30), fill=(255, 255, 255, int(255 * na)))
        d.text((nx + 118, ny + 108), "Direct Wi-Fi transfer • Saved to Downloads in 0.3s", font=font(FONT_REG, 24), fill=(56, 189, 248, int(255 * na)))
        
    specs_p = ease_out_cubic((lt - 0.70) / 0.35)
    if specs_p > 0.01:
        draw_pill_badge(canvas, W // 2, 1490, "• Chunked Streaming • 4K Video Ready", font(FONT_BOLD, 32),
                        bg=(18, 26, 44, 235), border=(249, 115, 22, 180), fg=(249, 115, 22, 255), pad_x=34, alpha=alpha * specs_p)

# Scene 5: Setup & GitHub CTA
def draw_scene_5(canvas, lt, s_dur, alpha):
    d = ImageDraw.Draw(canvas, "RGBA")
    t_head = ease_out_cubic(lt / 0.35)
    hy = int(lerp(240, 200, t_head))
    ha = int(255 * alpha * t_head)
    
    draw_centered_text(d, hy, "GET DROP BRIDGE", font(FONT_BLACK, 84), (248, 250, 252, ha))
    draw_centered_text(d, hy + 94, "Free & Open Source", font(FONT_BLACK, 68), (56, 189, 248, ha))
    
    # GitHub Repo Showcase Card
    card_p = ease_out_back((lt - 0.12) / 0.45, s=1.2)
    cw, ch = 880, 500
    cx = (W - cw) // 2
    cy = int(lerp(820, 740, card_p))
    ca = alpha * clamp01(card_p)
    
    # Shadow
    c_sh = Image.new("RGBA", (cw + 60, ch + 60), (0, 0, 0, 0))
    ImageDraw.Draw(c_sh).rounded_rectangle([30, 34, 30 + cw, 34 + ch], radius=32, fill=(0, 0, 0, 200))
    c_sh = c_sh.filter(ImageFilter.GaussianBlur(28))
    safe_composite(canvas, c_sh, cx - 30, cy - 34)
    
    d.rounded_rectangle([cx, cy, cx + cw, cy + ch], radius=32, fill=(16, 22, 36, int(245 * ca)), outline=(56, 189, 248, int(190 * ca)), width=3)
    
    # Card Header
    safe_composite(canvas, LOGO_ROUNDED.resize((96, 96), Image.Resampling.LANCZOS), cx + 45, cy + 45)
    d.text((cx + 165, cy + 48), "Drop Bridge", font=font(FONT_BLACK, 46), fill=(248, 250, 252, int(255 * ca)))
    d.text((cx + 168, cy + 104), "github.com/jorjanion/DropBridge", font=font(FONT_SEMI, 28), fill=(56, 189, 248, int(255 * ca)))
    
    # Features inside card
    card_bullets = [
        "• 30-Second Setup with single QR code",
        "• Zero iOS apps required (Official Apple Shortcut)",
        "• Windows 10, 11 & Linux supported",
        "• 100% Local Wi-Fi • No cloud accounts"
    ]
    for idx, b_text in enumerate(card_bullets):
        d.text((cx + 50, cy + 180 + idx * 60), b_text, font=font(FONT_BOLD, 30), fill=(226, 232, 240, int(245 * ca)))
        
    # Big CTA Button below card
    cta_p = ease_out_back((lt - 0.45) / 0.40, s=1.3)
    if cta_p > 0.01:
        bw, bh = 820, 126
        bx = (W - bw) // 2
        by = int(lerp(1380, 1310, cta_p))
        c_alpha = alpha * clamp01(cta_p)
        
        d.rounded_rectangle([bx, by, bx + bw, by + bh], radius=bh // 2, fill=(30, 58, 138, int(245 * c_alpha)), outline=(56, 189, 248, int(240 * c_alpha)), width=3)
        draw_centered_text(d, by + 34, "Star on GitHub & Download Free!", font(FONT_BLACK, 42), (255, 255, 255, int(255 * c_alpha)))
        
    link_p = ease_out_cubic((lt - 0.70) / 0.35)
    if link_p > 0.01:
        draw_pill_badge(canvas, W // 2, 1500, "Link in Bio • github.com/jorjanion/DropBridge", font(FONT_BOLD, 30),
                        bg=(18, 26, 44, 235), border=(74, 222, 128, 180), fg=(134, 239, 172, 255), pad_x=36, alpha=alpha * link_p)

SCENE_FNS = [draw_scene_1, draw_scene_2, draw_scene_3, draw_scene_4, draw_scene_5]
MIN_DURS = [4.2, 7.2, 8.4, 9.0, 8.4]
FADE_IN = 0.16
FADE_OUT = 0.14

def probe_audio_duration(path):
    res = subprocess.run([FFMPEG, "-i", path], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    for line in res.stderr.splitlines():
        if "Duration:" in line:
            dur_str = line.split("Duration:")[1].split(",")[0].strip()
            hh, mm, ss = dur_str.split(":")
            return int(hh) * 3600 + int(mm) * 60 + float(ss)
    raise RuntimeError(f"Could not read duration for {path}")

async def prepare_audio_clips():
    clips = []
    for i, line in enumerate(LINES):
        mp3 = os.path.join(SCRATCH_DIR, f"scene_{i+1}.mp3")
        wav = os.path.join(SCRATCH_DIR, f"scene_{i+1}.wav")
        if not os.path.exists(wav):
            comm = edge_tts.Communicate(line, "en-US-ChristopherNeural", rate="+10%")
            await comm.save(mp3)
            subprocess.run([
                FFMPEG, "-y", "-i", mp3,
                "-af", "silenceremove=start_periods=1:start_duration=0.02:start_threshold=-45dB,areverse,silenceremove=start_periods=1:start_duration=0.02:start_threshold=-45dB,areverse,loudnorm=I=-16:TP=-1.5:LRA=11",
                "-ar", "48000", "-ac", "2", wav
            ], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        dur = probe_audio_duration(wav)
        clips.append((wav, dur))
    return clips

def build_synced_audio_track(clips, scene_windows, total_duration):
    out_wav = os.path.join(SCRATCH_DIR, "master_audio.wav")
    inputs = []
    filter_parts = []
    mix_labels = []
    for i, ((wav_path, _), (s_start, _)) in enumerate(zip(clips, scene_windows)):
        inputs += ["-i", wav_path]
        delay_ms = int(round((s_start + 0.10) * 1000))
        lbl = f"a{i}"
        filter_parts.append(f"[{i}:a]adelay={delay_ms}|{delay_ms}[{lbl}]")
        mix_labels.append(f"[{lbl}]")
        
    filter_graph = ";".join(filter_parts) + f";{''.join(mix_labels)}amix=inputs={len(clips)}:duration=longest:normalize=0,apad=whole_dur={total_duration:.3f}[out]"
    subprocess.run([
        FFMPEG, "-y", *inputs,
        "-filter_complex", filter_graph,
        "-map", "[out]", "-t", f"{total_duration:.3f}",
        "-ar", "48000", "-ac", "2", out_wav
    ], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return out_wav

def render_video():
    print("[TikTok] Preparing neural voice clips...")
    clips = asyncio.run(prepare_audio_clips())
    
    scene_windows = []
    cur = 0.0
    for i, (_, clip_dur) in enumerate(clips):
        pad_tail = 0.45 if i == len(clips) - 1 else 0.28
        s_dur = max(MIN_DURS[i], clip_dur + 0.10 + pad_tail)
        scene_windows.append((cur, cur + s_dur))
        cur += s_dur
        
    total_dur = cur
    total_frames = int(math.ceil(total_dur * FPS))
    print(f"[TikTok] Scene timing: {[(round(a, 2), round(b, 2)) for a, b in scene_windows]} (total {total_dur:.2f}s)")
    
    master_wav = build_synced_audio_track(clips, scene_windows, total_dur)
    raw_video = os.path.join(SCRATCH_DIR, "raw_video.mp4")
    final_mp4 = os.path.join(OUT_DIR, "DropBridge-TikTok-EN.mp4")
    
    writer = cv2.VideoWriter(raw_video, cv2.CAP_MSMF, cv2.VideoWriter_fourcc(*"avc1"), FPS, (W, H))
    assert writer.isOpened(), f"Failed to open VideoWriter for {raw_video}"
    
    t0 = time.time()
    for f in range(total_frames):
        t = f / FPS
        canvas = Image.fromarray(BASE_BG, "RGB").convert("RGBA")
        
        # Moving ambient glowing orbs
        gx1 = int(W * 0.25 + math.sin(t * 0.55) * 180) - GLOW_CYAN.width // 2
        gy1 = int(H * 0.24 + math.cos(t * 0.45) * 140) - GLOW_CYAN.height // 2
        gx2 = int(W * 0.78 + math.cos(t * 0.50) * 160) - GLOW_INDIGO.width // 2
        gy2 = int(H * 0.72 + math.sin(t * 0.40) * 150) - GLOW_INDIGO.height // 2
        safe_composite(canvas, GLOW_CYAN, gx1, gy1)
        safe_composite(canvas, GLOW_INDIGO, gx2, gy2)
        
        # Single active scene
        active_idx = len(scene_windows) - 1
        for idx, (s_start, s_end) in enumerate(scene_windows):
            if s_start <= t < s_end:
                active_idx = idx
                break
                
        s_start, s_end = scene_windows[active_idx]
        s_dur = s_end - s_start
        lt = t - s_start
        
        a_in = 1.0 if active_idx == 0 else clamp01(lt / FADE_IN)
        a_out = 1.0 if active_idx == len(scene_windows) - 1 else clamp01((s_end - t) / FADE_OUT)
        a = min(a_in, a_out)
        
        if a > 0.01:
            SCENE_FNS[active_idx](canvas, lt, s_dur, a)
            
        # Smooth progress bar at bottom
        pd = ImageDraw.Draw(canvas, "RGBA")
        prog = clamp01(t / total_dur)
        pd.rounded_rectangle([120, 1820, W - 120, 1830], radius=5, fill=(255, 255, 255, 38))
        pd.rounded_rectangle([120, 1820, int(120 + (W - 240) * prog), 1830], radius=5, fill=(56, 189, 248, 230))
        
        rgb = canvas.convert("RGB")
        bgr = cv2.cvtColor(np.asarray(rgb), cv2.COLOR_RGB2BGR)
        writer.write(bgr)
        
    writer.release()
    
    # Mux video + master audio via ffmpeg
    print("[TikTok] Muxing video + audio with ffmpeg...")
    subprocess.run([
        FFMPEG, "-y",
        "-i", raw_video,
        "-i", master_wav,
        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18", "-preset", "fast",
        "-c:a", "aac", "-b:a", "192k",
        "-movflags", "+faststart",
        "-shortest",
        final_mp4
    ], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    
    elapsed = time.time() - t0
    size_mb = os.path.getsize(final_mp4) / (1024 * 1024)
    print(f"[TikTok] DONE in {elapsed:.1f}s -> {final_mp4} ({size_mb:.2f} MB, {total_dur:.1f}s)")

if __name__ == '__main__':
    render_video()
