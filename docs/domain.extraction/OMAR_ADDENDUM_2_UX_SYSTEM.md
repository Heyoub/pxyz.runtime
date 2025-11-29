# OMAR Addendum 2: UX System Features

> **Gap Analysis Result**: UI/UX system features that don't map to PXYZ operations but are critical configuration data loaded by the runtime.

---

## THEME SYSTEM

The theme system is NOT a workflow—it's **configuration data** loaded at runtime.

### Theme Configuration Schema

```xml
<schema id="theme_config">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="variant" type="enum" values="light,dark,high_contrast,muted,dyslexia"/>
  <field name="colors" type="object" required="true"/>
  <field name="fonts" type="object" required="true"/>
  <field name="spacing" type="object"/>
  <field name="is_system" type="boolean"/>
  <field name="created_by" type="uuid"/>
</schema>
```

### Theme Data Structure

```json
{
  "id": "theme-light-default",
  "name": "Light (Default)",
  "variant": "light",
  "colors": {
    "background": "#FFFFFF",
    "surface": "#F5F5F5",
    "primary": "#2563EB",
    "secondary": "#64748B",
    "accent": "#10B981",
    "text-primary": "#1F2937",
    "text-secondary": "#6B7280",
    "border": "#E5E7EB",
    "error": "#EF4444",
    "warning": "#F59E0B",
    "success": "#10B981",
    "info": "#3B82F6"
  },
  "fonts": {
    "primary": "Inter, system-ui, sans-serif",
    "monospace": "JetBrains Mono, monospace",
    "dyslexia": "OpenDyslexic, sans-serif",
    "base_size": "16px",
    "scale": 1.25,
    "line_height": 1.6,
    "letter_spacing": "normal"
  },
  "spacing": {
    "unit": "0.25rem",
    "scale": [0, 1, 2, 4, 6, 8, 12, 16, 24, 32, 48, 64]
  }
}
```

### Theme Variants

#### 1. Light Theme (Default)
```json
{
  "variant": "light",
  "colors": {
    "background": "#FFFFFF",
    "surface": "#F9FAFB",
    "text-primary": "#111827"
  }
}
```

#### 2. Dark Theme (OLED-Optimized)
```json
{
  "variant": "dark",
  "colors": {
    "background": "#000000",
    "surface": "#0A0A0A",
    "text-primary": "#F9FAFB",
    "primary": "#60A5FA"
  }
}
```

#### 3. High Contrast (WCAG AAA)
```json
{
  "variant": "high_contrast",
  "colors": {
    "background": "#FFFFFF",
    "text-primary": "#000000",
    "primary": "#0000FF",
    "error": "#CC0000",
    "border": "#000000"
  },
  "fonts": {
    "line_height": 1.8,
    "letter_spacing": "0.05em"
  }
}
```

#### 4. Muted (Low Saturation)
```json
{
  "variant": "muted",
  "colors": {
    "background": "#F5F5F5",
    "surface": "#EBEBEB",
    "primary": "#6B7280",
    "accent": "#9CA3AF"
  }
}
```

#### 5. Dyslexia Mode
```json
{
  "variant": "dyslexia",
  "colors": {
    "background": "#FAFAF8",
    "text-primary": "#0F0F0F"
  },
  "fonts": {
    "primary": "OpenDyslexic, sans-serif",
    "base_size": "18px",
    "line_height": 1.8,
    "letter_spacing": "0.08em",
    "word_spacing": "0.15em"
  }
}
```

### Theme Operations (User Preferences)

```yaml
# These are preference updates, not workflows
theme_set_user_preference: 0x1500
theme_get_user_preference: 0x1501
theme_toggle: 0x1502
theme_custom_create: 0x1503
```

### Theme Application Workflow

```xml
<workflow id="user_theme_preference">
  <entry p="user" x="set_theme" node="validate_theme"/>
  
  <nodes>
    <node id="validate_theme" kind="transform">
      <validate>
        <field name="theme_id" type="uuid" required="true"/>
      </validate>
    </node>
    
    <node id="check_theme_exists" kind="external" op="0x1501">
      <load_theme id="$input.theme_id"/>
    </node>
    
    <node id="update_preference" kind="external" op="0x0910">
      <event>
        <type>user.preference.updated</type>
        <data>
          <field name="user_id" value="$token.sub"/>
          <field name="preference_key" value="theme"/>
          <field name="preference_value" value="$input.theme_id"/>
        </data>
      </event>
    </node>
    
    <node id="signal_ui_update" kind="signal">
      <signal>theme_changed</signal>
      <data>
        <field name="theme" value="$theme"/>
      </data>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_theme" to="check_theme_exists"><when><always/></when></edge>
    <edge from="check_theme_exists" to="update_preference"><when><always/></when></edge>
    <edge from="update_preference" to="signal_ui_update"><when><always/></when></edge>
    <edge from="signal_ui_update" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### CSS Custom Properties Generation

The theme config generates CSS custom properties:

```css
/* Auto-generated from theme config */
:root[data-theme="light"] {
  --color-background: #FFFFFF;
  --color-surface: #F9FAFB;
  --color-primary: #2563EB;
  --font-primary: Inter, system-ui, sans-serif;
  --font-size-base: 16px;
  --line-height-base: 1.6;
  --spacing-unit: 0.25rem;
}

:root[data-theme="dark"] {
  --color-background: #000000;
  --color-surface: #0A0A0A;
  --color-primary: #60A5FA;
  /* ... */
}
```

---

## NOTIFICATION UI SYSTEM

Notifications have multiple presentation modes defined as **configuration**, not workflows.

### Notification Display Config

```xml
<schema id="notification_display_config">
  <field name="type" type="enum" values="toast,shelf,inline,email,push"/>
  <field name="priority" type="enum" values="low,medium,high,critical"/>
  <field name="auto_dismiss_ms" type="integer"/>
  <field name="can_dismiss" type="boolean"/>
  <field name="show_actions" type="boolean"/>
  <field name="sound_enabled" type="boolean"/>
  <field name="badge_enabled" type="boolean"/>
</schema>
```

### Notification Type Specifications

#### 1. Toast Notifications
```json
{
  "type": "toast",
  "display": {
    "position": "top-right",
    "max_width": "400px",
    "auto_dismiss_ms": 4000,
    "animation": "slide-in-right",
    "stack_limit": 3
  },
  "triggers": [
    "task.completed",
    "file.uploaded",
    "email.sent"
  ]
}
```

#### 2. Shelf Notifications
```json
{
  "type": "shelf",
  "display": {
    "position": "header-dropdown",
    "max_items": 50,
    "auto_dismiss_ms": null,
    "show_badge_count": true,
    "group_by": "type"
  },
  "triggers": [
    "task.assigned",
    "workflow.milestone_reached",
    "invoice.overdue",
    "approval.requested"
  ]
}
```

#### 3. Inline/Contextual Notifications
```json
{
  "type": "inline",
  "display": {
    "glow_color": "#F59E0B",
    "glow_duration_ms": 2000,
    "show_badge": true,
    "pulse_animation": true
  },
  "triggers": [
    "task.overdue",
    "contact.cooling",
    "workflow.at_risk"
  ]
}
```

#### 4. Email Digest
```json
{
  "type": "email",
  "display": {
    "frequency": "daily",
    "time": "08:00",
    "timezone": "user_preference",
    "group_by": ["type", "priority"],
    "max_items": 50
  },
  "triggers": "all_unacknowledged"
}
```

#### 5. Mobile Push
```json
{
  "type": "push",
  "display": {
    "title_template": "{{event.entity_type}} {{event.action}}",
    "body_template": "{{event.summary}}",
    "show_actions": true,
    "sound": "default",
    "badge_increment": true
  },
  "triggers": [
    "mention.user",
    "approval.urgent",
    "message.received"
  ],
  "quiet_hours": {
    "enabled": true,
    "start": "22:00",
    "end": "08:00",
    "critical_only": true
  }
}
```

### Notification Routing Logic

```xml
<workflow id="notification_route">
  <entry p="notification" x="send" node="classify_priority"/>
  
  <nodes>
    <node id="classify_priority" kind="external" op="0x0801">
      <classify>
        <input from="$notification.content"/>
        <categories>critical,high,medium,low</categories>
      </classify>
    </node>
    
    <node id="check_quiet_hours" kind="auth">
      <predicate id="is_within_quiet_hours">
        <and>
          <eq left="$user.quiet_hours_enabled" right="true"/>
          <gte left="$now.hour" right="$user.quiet_hours_start"/>
          <lte left="$now.hour" right="$user.quiet_hours_end"/>
        </and>
      </predicate>
    </node>
    
    <node id="route_to_channels" kind="transform">
      <compute>
        <var name="channels" value="determine_channels($notification.priority, $user.preferences, $is_quiet_hours)"/>
      </compute>
    </node>
    
    <node id="send_toast" kind="signal">
      <when>
        <contains left="$channels" right="toast"/>
      </when>
      <signal>show_toast</signal>
      <data>
        <field name="message" value="$notification.message"/>
        <field name="type" value="$notification.type"/>
      </data>
    </node>
    
    <node id="add_to_shelf" kind="external" op="0x0910">
      <when>
        <contains left="$channels" right="shelf"/>
      </when>
      <event>
        <type>notification.shelf.added</type>
        <data>
          <field name="user_id" value="$notification.user_id"/>
          <field name="content" value="$notification"/>
        </data>
      </event>
    </node>
    
    <node id="send_push" kind="external" op="0x1600">
      <when>
        <contains left="$channels" right="push"/>
      </when>
      <push_notification>
        <to value="$notification.user_id"/>
        <title value="$notification.title"/>
        <body value="$notification.body"/>
      </push_notification>
    </node>
    
    <node id="add_to_digest" kind="external" op="0x0910">
      <when>
        <contains left="$channels" right="email"/>
      </when>
      <event>
        <type>notification.digest.queued</type>
        <data>
          <field name="user_id" value="$notification.user_id"/>
          <field name="content" value="$notification"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="classify_priority" to="check_quiet_hours"><when><always/></when></edge>
    <edge from="check_quiet_hours" to="route_to_channels"><when><always/></when></edge>
    <edge from="route_to_channels" to="send_toast">
      <when><contains left="$channels" right="toast"/></when>
    </edge>
    <edge from="route_to_channels" to="add_to_shelf">
      <when><contains left="$channels" right="shelf"/></when>
    </edge>
    <edge from="route_to_channels" to="send_push">
      <when><contains left="$channels" right="push"/></when>
    </edge>
    <edge from="route_to_channels" to="add_to_digest">
      <when><contains left="$channels" right="email"/></when>
    </edge>
    <edge from="send_toast" to="add_to_shelf"><when><always/></when></edge>
    <edge from="add_to_shelf" to="send_push"><when><always/></when></edge>
    <edge from="send_push" to="add_to_digest"><when><always/></when></edge>
    <edge from="add_to_digest" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## STRATEGIC FRICTION SYSTEM

Strategic friction is **UX configuration** that injects confirmation prompts at critical decision points.

### Friction Point Configuration

```xml
<schema id="friction_point">
  <field name="id" type="uuid" required="true"/>
  <field name="action_id" type="string" required="true"/>
  <field name="prompt_type" type="enum" values="confirm,acknowledge,review"/>
  <field name="message" type="string" required="true"/>
  <field name="severity" type="enum" values="info,warning,critical"/>
  <field name="can_bypass" type="boolean"/>
  <field name="bypass_role" type="string"/>
</schema>
```

### Friction Points Data

```json
[
  {
    "id": "friction-portal-approve-design",
    "action_id": "portal.approve_deliverable",
    "prompt_type": "confirm",
    "message": "You're about to approve the final design. This will move the project to development. Are you ready to proceed?",
    "severity": "warning",
    "can_bypass": false,
    "requires_explicit_confirmation": true
  },
  {
    "id": "friction-portal-upload-contract",
    "action_id": "portal.upload_document",
    "prompt_type": "acknowledge",
    "message": "Uploading contracts? Please ensure:\n• All signatures are visible\n• All pages are included\n• File is legible",
    "severity": "info",
    "can_bypass": true,
    "bypass_text": "I've verified the above"
  },
  {
    "id": "friction-workflow-skip-step",
    "action_id": "workflow.skip_step",
    "prompt_type": "review",
    "message": "Skipping this step may affect project quality. Reason for skipping?",
    "severity": "warning",
    "can_bypass": false,
    "requires_reason": true
  },
  {
    "id": "friction-invoice-send-large",
    "action_id": "invoice.send",
    "prompt_type": "confirm",
    "message": "This invoice is for ${{amount}}. Double-check line items before sending.",
    "severity": "critical",
    "can_bypass": false,
    "condition": "amount > 10000"
  },
  {
    "id": "friction-contact-delete",
    "action_id": "contact.delete",
    "prompt_type": "confirm",
    "message": "Delete {{contact.name}}? This will remove {{linked_count}} linked items:\n• {{task_count}} tasks\n• {{email_count}} emails\n• {{document_count}} documents",
    "severity": "critical",
    "can_bypass": false,
    "requires_explicit_type": "DELETE"
  }
]
```

### Friction Workflow Integration

```xml
<workflow id="portal_approve_with_friction">
  <entry p="portal" x="approve" node="load_deliverable"/>
  
  <nodes>
    <node id="load_deliverable" kind="external" op="0x0902"/>
    
    <node id="check_friction_point" kind="transform">
      <load_friction action_id="portal.approve_deliverable"/>
    </node>
    
    <node id="show_confirmation" kind="render">
      <when>
        <not_null left="$friction_point"/>
      </when>
      <template ref="friction_confirmation">
        <message value="$friction_point.message"/>
        <severity value="$friction_point.severity"/>
        <actions>
          <action label="Cancel" value="cancel"/>
          <action label="Confirm" value="confirm" primary="true"/>
        </actions>
      </template>
    </node>
    
    <node id="await_user_response" kind="signal">
      <await signal="friction_response"/>
    </node>
    
    <node id="check_confirmed" kind="auth">
      <require>
        <eq left="$user_response" right="confirm"/>
      </require>
    </node>
    
    <node id="execute_approval" kind="external" op="0x0920">
      <approve deliverable_id="$deliverable.id"/>
      <event type="portal.deliverable_approved"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_deliverable" to="check_friction_point"><when><always/></when></edge>
    <edge from="check_friction_point" to="show_confirmation">
      <when><not_null left="$friction_point"/></when>
    </edge>
    <edge from="check_friction_point" to="execute_approval">
      <when><null left="$friction_point"/></when>
    </edge>
    <edge from="show_confirmation" to="await_user_response"><when><always/></when></edge>
    <edge from="await_user_response" to="check_confirmed"><when><always/></when></edge>
    <edge from="check_confirmed" to="execute_approval"><when><always/></when></edge>
    <edge from="execute_approval" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## ACCESSIBILITY SYSTEM

Accessibility is configuration + runtime enforcement.

### Accessibility Config

```json
{
  "wcag_level": "AAA",
  "features": {
    "screen_reader_optimized": true,
    "keyboard_navigation": true,
    "focus_indicators": "enhanced",
    "skip_links": true,
    "aria_labels": "comprehensive",
    "reduced_motion": "user_preference"
  },
  "color_contrast": {
    "minimum_ratio": 7.0,
    "large_text_ratio": 4.5,
    "enforce_at_runtime": true
  },
  "font_sizing": {
    "minimum_size": "16px",
    "scale_with_browser": true,
    "max_zoom_level": 200
  },
  "keyboard_shortcuts": {
    "enabled": true,
    "customizable": true,
    "show_help": "Ctrl+/",
    "conflicts": "prevent"
  }
}
```

### Accessibility Validation (Compile-Time)

```xml
<workflow id="template_accessibility_check">
  <!-- Runs during XML → Binary compilation -->
  <entry p="template" x="validate_accessibility" node="parse_template"/>
  
  <nodes>
    <node id="parse_template" kind="transform">
      <parse html="$template.content"/>
    </node>
    
    <node id="check_contrast" kind="transform">
      <for_each element="$dom_elements">
        <compute>
          <var name="contrast_ratio" value="calculate_contrast($element.foreground, $element.background)"/>
        </compute>
        <when>
          <lt left="$contrast_ratio" right="7.0"/>
        </when>
        <error>
          Element {{element.selector}} has contrast ratio {{contrast_ratio}}, minimum is 7.0 (WCAG AAA)
        </error>
      </for_each>
    </node>
    
    <node id="check_alt_text" kind="transform">
      <for_each element="img">
        <when>
          <or>
            <null left="$element.alt"/>
            <empty left="$element.alt"/>
          </or>
        </when>
        <error>
          Image {{element.src}} missing alt text
        </error>
      </for_each>
    </node>
    
    <node id="check_aria_labels" kind="transform">
      <for_each element="button,a,input">
        <when>
          <and>
            <empty left="$element.text_content"/>
            <or>
              <null left="$element.aria_label"/>
              <null left="$element.aria_labelledby"/>
            </or>
          </and>
        </when>
        <error>
          {{element.tag}} missing accessible label
        </error>
      </for_each>
    </node>
    
    <node id="check_heading_hierarchy" kind="transform">
      <validate_heading_order elements="h1,h2,h3,h4,h5,h6"/>
      <when>
        <has_skipped_levels left="$heading_order"/>
      </when>
      <warning>
        Heading hierarchy has skipped levels
      </warning>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
</workflow>
```

---

## RTL LANGUAGE SUPPORT

RTL is configuration that affects rendering.

### RTL Config

```json
{
  "rtl_languages": ["ar", "he", "fa", "ur"],
  "layout": {
    "auto_detect": true,
    "direction_attribute": "dir",
    "flip_icons": true,
    "flip_layouts": true
  },
  "typography": {
    "font_families": {
      "ar": "Noto Sans Arabic, sans-serif",
      "he": "Noto Sans Hebrew, sans-serif",
      "fa": "Noto Sans Arabic, sans-serif"
    },
    "text_align": "start"
  }
}
```

### RTL Template Rendering

```xml
<workflow id="render_with_rtl">
  <entry p="template" x="render" node="detect_language"/>
  
  <nodes>
    <node id="detect_language" kind="transform">
      <compute>
        <var name="lang" value="$user.language || $browser.language"/>
        <var name="is_rtl" value="contains($rtl_languages, $lang)"/>
      </compute>
    </node>
    
    <node id="apply_rtl_styles" kind="transform">
      <when>
        <eq left="$is_rtl" right="true"/>
      </when>
      <modify_dom>
        <set_attribute name="dir" value="rtl"/>
        <add_class value="rtl-layout"/>
        <flip_icons/>
        <swap_margins/>
      </modify_dom>
    </node>
    
    <node id="render_template" kind="render">
      <template ref="$template_id"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
</workflow>
```

---

## INTERACTION AUDITING (2-3 Click Principle)

This is a **monitoring system** that tracks user journeys.

### Click Path Tracking

```xml
<schema id="click_path">
  <field name="id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="session_id" type="uuid" required="true"/>
  <field name="goal" type="string" required="true"/>
  <field name="clicks" type="array" required="true"/>
  <field name="duration_ms" type="integer"/>
  <field name="success" type="boolean"/>
</schema>
```

### Background Tracking Workflow

```xml
<workflow id="track_click_path">
  <!-- Runs on every user action -->
  <entry p="analytics" x="track_click" node="append_click"/>
  
  <nodes>
    <node id="append_click" kind="external" op="0x0910">
      <event>
        <type>analytics.click</type>
        <data>
          <field name="session_id" value="$session.id"/>
          <field name="element" value="$input.element"/>
          <field name="timestamp" value="$now"/>
          <field name="page" value="$current_page"/>
        </data>
      </event>
    </node>
    
    <node id="detect_goal_completion" kind="transform">
      <check>
        <if element="$input.element" matches="goal_completion_selector"/>
        <then mark_session_complete="true"/>
      </check>
    </node>
    
    <node id="calculate_click_count" kind="transform">
      <when>
        <eq left="$session_complete" right="true"/>
      </when>
      <compute>
        <var name="click_count" value="count($session.clicks)"/>
        <var name="duration_ms" value="$session.end - $session.start"/>
      </compute>
    </node>
    
    <node id="check_violation" kind="auth">
      <when>
        <gt left="$click_count" right="3"/>
      </when>
      <flag_ux_violation>
        <goal value="$session.goal"/>
        <click_count value="$click_count"/>
        <path value="$session.clicks"/>
      </flag_ux_violation>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
</workflow>
```

---

## SUMMARY

This addendum covers **5 major UX system categories**:

1. **Theme System**: 5 variants (light, dark, high-contrast, muted, dyslexia), CSS custom properties, user preferences
2. **Notification UI**: 5 display types (toast, shelf, inline, email, push), routing logic, quiet hours
3. **Strategic Friction**: Confirmation prompts at critical decision points, compliance-driven UX
4. **Accessibility**: WCAG AAA compliance, compile-time validation, runtime enforcement, keyboard navigation
5. **RTL Support**: Auto-detection, layout flipping, language-specific fonts

**Key Insight**: These are NOT workflows—they're **configuration data** and **runtime behaviors** that the WASM runtime consumes.

### How It Fits in OMAR

```
User Action
    ↓
WASM Runtime loads:
    • graph.bin (workflows)
    • theme-config.json (colors, fonts)
    • friction-points.json (UX confirmations)
    • notification-rules.json (display logic)
    • accessibility-config.json (validation rules)
    ↓
Runtime applies config to execution:
    • Themes → CSS variables
    • Friction → Inject confirmation nodes
    • Notifications → Route to channels
    • Accessibility → Validate output
    ↓
Result: Accessible, themed, strategically-frictionful UX
```

All configuration is **data**, not code. The 700-line runtime simply loads and applies it.
