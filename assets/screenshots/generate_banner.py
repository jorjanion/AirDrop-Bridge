import os
import math
from PIL import Image, ImageDraw, ImageFont, ImageFilter

def draw_wifi_icon(size=70, color=(56, 189, 248, 255)):
    # Render at 2x and scale down for antialiasing
    s2 = size * 2
    im = Image.new('RGBA', (s2, s2), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    cx, cy = s2 // 2, int(s2 * 0.72)
    
    # Dot
    r0 = 7
    d.ellipse([cx - r0, cy - r0, cx + r0, cy + r0], fill=color)
    
    # Arc 1
    r1 = 28
    d.arc([cx - r1, cy - r1, cx + r1, cy + r1], start=215, end=325, fill=color, width=5)
    
    # Arc 2
    r2 = 48
    d.arc([cx - r2, cy - r2, cx + r2, cy + r2], start=215, end=325, fill=color, width=5)
    
    # Arc 3
    r3 = 68
    d.arc([cx - r3, cy - r3, cx + r3, cy + r3], start=215, end=325, fill=color, width=5)
    
    return im.resize((size, size), Image.Resampling.LANCZOS)

def create_banner():
    W, H = 1920, 1080
    
    # Base Canvas
    base = Image.new('RGBA', (W, H), (10, 14, 26, 255))
    draw = ImageDraw.Draw(base)
    
    # Vertical background gradient
    for y in range(H):
        ratio = y / H
        r = int(9 + (18 - 9) * ratio)
        g = int(13 + (26 - 13) * ratio)
        b = int(24 + (44 - 24) * ratio)
        draw.line([(0, y), (W, y)], fill=(r, g, b, 255))
        
    # Ambient Glow Orbs
    glow = Image.new('RGBA', (W, H), (0, 0, 0, 0))
    g_draw = ImageDraw.Draw(glow)
    
    # Soft cyan orb behind iPhone
    g_draw.ellipse([80, 220, 680, 920], fill=(56, 189, 248, 55))
    # Soft indigo orb behind Windows PC
    g_draw.ellipse([1180, 200, 1820, 920], fill=(99, 102, 241, 50))
    # Subtle central blue bridge glow
    g_draw.ellipse([640, 420, 1280, 700], fill=(14, 165, 233, 35))
    
    glow = glow.filter(ImageFilter.GaussianBlur(100))
    base = Image.alpha_composite(base, glow)
    
    # Fonts
    font_dir = os.path.join(os.environ.get('WINDIR', 'C:\\Windows'), 'Fonts')
    font_bold = os.path.join(font_dir, 'segoeuib.ttf')
    font_reg = os.path.join(font_dir, 'segoeui.ttf')
    
    f_title = ImageFont.truetype(font_bold, 54)
    f_subtitle = ImageFont.truetype(font_reg, 24)
    f_header_logo = ImageFont.truetype(font_bold, 32)
    f_step_label = ImageFont.truetype(font_bold, 15)
    f_badge = ImageFont.truetype(font_bold, 17)
    f_callout = ImageFont.truetype(font_bold, 14)
    f_transfer_title = ImageFont.truetype(font_bold, 22)
    f_transfer_sub = ImageFont.truetype(font_reg, 16)
    f_transfer_pill = ImageFont.truetype(font_bold, 14)
    f_notif_title = ImageFont.truetype(font_bold, 16)
    f_notif_body = ImageFont.truetype(font_reg, 15)
    f_notif_time = ImageFont.truetype(font_reg, 13)

    # 1. Header (Logo + Title + Subtitle)
    icon_path = 'resources/icon.png'
    if os.path.exists(icon_path):
        icon = Image.open(icon_path).convert('RGBA')
        icon_sm = icon.resize((56, 56), Image.Resampling.LANCZOS)
        base.paste(icon_sm, (90, 48), icon_sm)
        
    draw = ImageDraw.Draw(base)
    draw.text((160, 58), "AirDrop Bridge", font=f_header_logo, fill=(248, 250, 252, 255))
    
    # Pill next to logo
    draw.rounded_rectangle([410, 60, 550, 88], radius=14, fill=(30, 41, 59, 230), outline=(56, 189, 248, 140), width=1)
    draw.text((426, 65), "OPEN SOURCE", font=f_callout, fill=(56, 189, 248, 255))
    
    # Main Headline
    draw.text((90, 122), "AirDrop to PC with ZERO Apps on iOS", font=f_title, fill=(255, 255, 255, 255))
    
    # Subtitle
    draw.text((92, 192), "Native Apple Shortcuts  •  Direct Local Wi-Fi  •  Built with Rust  •  No Cloud, 100% Private", font=f_subtitle, fill=(148, 163, 184, 255))
    
    # 2. Left Column: iPhone Mockup with Real Share Sheet
    share_path = 'assets/screenshots/ios-share-sheet.png'
    phone_x, phone_y = 110, 285
    p_w, p_h = 328, 672
    
    # Step label above iPhone
    draw.text((phone_x + 10, phone_y - 30), "STEP 1: TAP SHARE ON IPHONE", font=f_step_label, fill=(56, 189, 248, 255))
    
    if os.path.exists(share_path):
        ss = Image.open(share_path).convert('RGBA')
        s_w, s_h = 304, 648
        ss = ss.resize((s_w, s_h), Image.Resampling.LANCZOS)
        
        # Phone body frame
        bezel = 12
        phone_body = Image.new('RGBA', (p_w, p_h), (0, 0, 0, 0))
        p_draw = ImageDraw.Draw(phone_body)
        
        # Dark Titanium bezel
        p_draw.rounded_rectangle([0, 0, p_w, p_h], radius=42, fill=(26, 29, 36, 255), outline=(71, 85, 105, 240), width=2)
        
        # Screen with rounded mask
        screen_mask = Image.new('L', (s_w, s_h), 0)
        sm_draw = ImageDraw.Draw(screen_mask)
        sm_draw.rounded_rectangle([0, 0, s_w, s_h], radius=30, fill=255)
        phone_body.paste(ss, (bezel, bezel), screen_mask)
        
        # EXACT Highlight rectangle on "AirDrop Bridge" row in the share sheet
        hl_y1 = bezel + 528
        hl_y2 = bezel + 570
        p_draw.rounded_rectangle([bezel + 6, hl_y1, bezel + s_w - 6, hl_y2], radius=10, outline=(56, 189, 248, 255), width=2)
        
        # Dynamic Island at top
        di_w, di_h = 80, 20
        di_x = (p_w - di_w) // 2
        p_draw.rounded_rectangle([di_x, bezel + 6, di_x + di_w, bezel + 6 + di_h], radius=10, fill=(0, 0, 0, 255))
        
        # Phone Drop Shadow
        shadow = Image.new('RGBA', (p_w + 80, p_h + 80), (0, 0, 0, 0))
        s_draw = ImageDraw.Draw(shadow)
        s_draw.rounded_rectangle([40, 45, 40 + p_w, 45 + p_h], radius=42, fill=(0, 0, 0, 170))
        shadow = shadow.filter(ImageFilter.GaussianBlur(25))
        
        base.paste(shadow, (phone_x - 40, phone_y - 45), shadow)
        base.paste(phone_body, (phone_x, phone_y), phone_body)
        
        # Callout tag pointing directly to the highlighted AirDrop Bridge row
        tag_x = phone_x + p_w + 14
        tag_y = phone_y + hl_y1 + 4
        draw.rounded_rectangle([tag_x, tag_y, tag_x + 195, tag_y + 34], radius=17, fill=(15, 23, 42, 245), outline=(56, 189, 248, 220), width=1)
        draw.text((tag_x + 14, tag_y + 7), "← Native Apple Shortcut", font=f_callout, fill=(56, 189, 248, 255))

    # 3. Right Column: Windows PC Window + Toast Notification
    pair_path = 'assets/screenshots/pairing-window.png'
    win_x = 1350
    win_y = 285
    
    # Step label above PC
    draw.text((win_x + 10, win_y - 30), "STEP 2: RECEIVE ON PC", font=f_step_label, fill=(165, 180, 252, 255))
    
    if os.path.exists(pair_path):
        pw_scaled_h = 490
        pw_scaled_w = int(461 * (pw_scaled_h / 611))  # ~370
        pw = Image.open(pair_path).convert('RGBA')
        pw = pw.resize((pw_scaled_w, pw_scaled_h), Image.Resampling.LANCZOS)
        
        # Window wrapper
        win_wrapper = Image.new('RGBA', (pw_scaled_w, pw_scaled_h), (0, 0, 0, 0))
        win_mask = Image.new('L', (pw_scaled_w, pw_scaled_h), 0)
        wm_draw = ImageDraw.Draw(win_mask)
        wm_draw.rounded_rectangle([0, 0, pw_scaled_w, pw_scaled_h], radius=12, fill=255)
        win_wrapper.paste(pw, (0, 0), win_mask)
        
        # 1px acrylic border
        ww_draw = ImageDraw.Draw(win_wrapper)
        ww_draw.rounded_rectangle([0, 0, pw_scaled_w - 1, pw_scaled_h - 1], radius=12, outline=(255, 255, 255, 45), width=1)
        
        # Shadow for PC window
        w_shadow = Image.new('RGBA', (pw_scaled_w + 80, pw_scaled_h + 80), (0, 0, 0, 0))
        ws_draw = ImageDraw.Draw(w_shadow)
        ws_draw.rounded_rectangle([40, 45, 40 + pw_scaled_w, 45 + pw_scaled_h], radius=12, fill=(0, 0, 0, 180))
        w_shadow = w_shadow.filter(ImageFilter.GaussianBlur(26))
        
        base.paste(w_shadow, (win_x - 40, win_y - 45), w_shadow)
        base.paste(win_wrapper, (win_x, win_y), win_wrapper)
        
        # Windows 11 Notification Toast Card (positioned right underneath)
        notif_w = 440
        notif_h = 88
        notif_x = win_x - (notif_w - pw_scaled_w) // 2
        notif_y = win_y + pw_scaled_h + 20
        
        notif_img = Image.new('RGBA', (notif_w, notif_h), (0, 0, 0, 0))
        n_draw = ImageDraw.Draw(notif_img)
        # Acrylic card background
        n_draw.rounded_rectangle([0, 0, notif_w - 1, notif_h - 1], radius=14, fill=(22, 28, 44, 245), outline=(56, 189, 248, 130), width=1)
        
        if os.path.exists(icon_path):
            n_icon = Image.open(icon_path).convert('RGBA').resize((36, 36), Image.Resampling.LANCZOS)
            notif_img.paste(n_icon, (16, 14), n_icon)
            
        n_draw.text((64, 14), "AirDrop Bridge", font=f_notif_title, fill=(241, 245, 249, 255))
        n_draw.text((362, 16), "Just now", font=f_notif_time, fill=(148, 163, 184, 255))
        n_draw.text((64, 36), "Photo received: IMG_4820.HEIC (12.4 MB)", font=f_notif_body, fill=(255, 255, 255, 255))
        n_draw.text((64, 58), "Direct Wi-Fi transfer • Saved to Downloads", font=f_notif_time, fill=(56, 189, 248, 255))
        
        # Toast shadow
        n_shadow = Image.new('RGBA', (notif_w + 60, notif_h + 60), (0, 0, 0, 0))
        ns_draw = ImageDraw.Draw(n_shadow)
        ns_draw.rounded_rectangle([30, 32, 30 + notif_w, 32 + notif_h], radius=14, fill=(0, 0, 0, 190))
        n_shadow = n_shadow.filter(ImageFilter.GaussianBlur(18))
        
        base.paste(n_shadow, (notif_x - 30, notif_y - 32), n_shadow)
        base.paste(notif_img, (notif_x, notif_y), notif_img)

    # 4. Center Bridge Section: Transfer Wave, Speed Stats, Connection Beam
    c_x1 = phone_x + p_w + 220
    c_x2 = win_x - 40
    c_mid_x = (c_x1 + c_x2) // 2
    c_y = 515
    
    bridge_draw = ImageDraw.Draw(base)
    
    # Glowing dashed line connecting Phone -> Center -> PC
    for x in range(phone_x + p_w + 15, win_x - 25, 20):
        # Skip region under center card
        if x > c_mid_x - 195 and x < c_mid_x + 195:
            continue
        bridge_draw.line([(x, c_y), (x + 10, c_y)], fill=(56, 189, 248, 170), width=3)
        
    # Arrow head pointing to PC
    arrow_x = win_x - 25
    bridge_draw.polygon([(arrow_x, c_y - 8), (arrow_x + 14, c_y), (arrow_x, c_y + 8)], fill=(56, 189, 248, 240))
    
    # Center Transfer Glass Card
    card_w, card_h = 370, 246
    card_x = c_mid_x - card_w // 2
    card_y = c_y - card_h // 2
    
    card_img = Image.new('RGBA', (card_w, card_h), (0, 0, 0, 0))
    cd = ImageDraw.Draw(card_img)
    cd.rounded_rectangle([0, 0, card_w - 1, card_h - 1], radius=20, fill=(15, 23, 42, 245), outline=(56, 189, 248, 150), width=1)
    
    # Draw vector Wi-Fi waves
    wifi_icon = draw_wifi_icon(size=56, color=(56, 189, 248, 255))
    card_img.paste(wifi_icon, (card_w // 2 - 28, 16), wifi_icon)
    
    # Transfer card texts
    cd.text((card_w // 2 - 105, 80), "Instant P2P Transfer", font=f_transfer_title, fill=(255, 255, 255, 255))
    cd.text((card_w // 2 - 118, 114), "Local Wi-Fi  •  Zero Cloud Storage", font=f_transfer_sub, fill=(148, 163, 184, 255))
    
    # 2 Clean Feature pills inside card without missing glyphs
    # Pill 1: Chunked Disk Streaming
    p1_text = "• Chunked Streaming (4K Video Ready)"
    p1_bbox = cd.textbbox((0, 0), p1_text, font=f_transfer_pill)
    p1_x = (card_w - (p1_bbox[2] - p1_bbox[0])) // 2
    cd.rounded_rectangle([30, 150, card_w - 30, 186], radius=10, fill=(30, 41, 59, 210), outline=(56, 189, 248, 90), width=1)
    cd.text((p1_x, 158), p1_text, font=f_transfer_pill, fill=(56, 189, 248, 255))
    
    # Pill 2: Memory footprint
    p2_text = "• Ultra-Lightweight (<15 MB RAM)"
    p2_bbox = cd.textbbox((0, 0), p2_text, font=f_transfer_pill)
    p2_x = (card_w - (p2_bbox[2] - p2_bbox[0])) // 2
    cd.rounded_rectangle([30, 194, card_w - 30, 230], radius=10, fill=(30, 41, 59, 210), outline=(249, 115, 22, 90), width=1)
    cd.text((p2_x, 202), p2_text, font=f_transfer_pill, fill=(249, 115, 22, 255))
    
    # Center card shadow
    c_shadow = Image.new('RGBA', (card_w + 60, card_h + 60), (0, 0, 0, 0))
    cs_draw = ImageDraw.Draw(c_shadow)
    cs_draw.rounded_rectangle([30, 32, 30 + card_w, 32 + card_h], radius=20, fill=(0, 0, 0, 190))
    c_shadow = c_shadow.filter(ImageFilter.GaussianBlur(22))
    
    base.paste(c_shadow, (card_x - 30, card_y - 32), c_shadow)
    base.paste(card_img, (card_x, card_y), card_img)
    
    # 5. Bottom Feature Badges (cleanly spaced, no overlaps)
    badge_items = [
        ("• 100% Local & Private", (16, 185, 129)),      # Emerald
        ("• Zero iOS Apps Needed", (56, 189, 248)),      # Cyan
        ("• Built in Rust 2024", (249, 115, 22)),        # Orange
        ("• Bidirectional Clipboard", (168, 85, 247)),   # Purple
        ("• Free & Open Source", (234, 179, 8))          # Amber
    ]
    
    b_y = 1004
    badge_widths = [draw.textbbox((0, 0), item[0], font=f_badge)[2] + 36 for item in badge_items]
    spacing = 18
    total_w = sum(badge_widths) + spacing * (len(badge_items) - 1)
    start_x = (W - total_w) // 2
    
    cur_x = start_x
    for i, (text, color) in enumerate(badge_items):
        pw = badge_widths[i]
        ph = 42
        draw.rounded_rectangle([cur_x, b_y, cur_x + pw, b_y + ph], radius=21, fill=(22, 28, 44, 230), outline=(*color, 140), width=1)
        draw.text((cur_x + 18, b_y + 10), text, font=f_badge, fill=(241, 245, 249, 255))
        cur_x += pw + spacing
        
    # Save image
    output_path = 'assets/screenshots/reddit-promo.png'
    base.save(output_path, 'PNG', quality=95)
    print(f"Polished banner saved to {output_path}")

if __name__ == '__main__':
    create_banner()
