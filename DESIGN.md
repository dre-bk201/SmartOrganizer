---
name: SmartOrganizer Flow
description: A crimson desktop workspace for readable folder automation.
colors:
  acrylic-light: "#ff9b70"
  acrylic-ink: "#25151a"
  acrylic-text: "#f6f1f3"
  acrylic-muted: "#c6bfc8"
  acrylic-base: "#171b24"
  acrylic-surface: "#1c202a"
  acrylic-border: "#42424e"
  acrylic-dialog: "#1c1d29"
  crimson: "#a32949"
  paper: "#fcf9fa"
  crimson-surface: "#f5edef"
  crimson-border: "#e8d9dd"
  crimson-text: "#422b32"
  crimson-muted: "#80616b"
  crimson-selected: "#f5dfe6"
  light-button-ink: "#fff"
  dark-paper: "#281e23"
  dark-surface: "#20171c"
  dark-border: "#57404b"
  dark-text: "#f7e8ed"
  dark-muted: "#d2b3be"
  dark-accent: "#efa2b7"
  dark-selected: "#51303d"
typography:
  headline:
    fontFamily: "Segoe UI, sans-serif"
    fontSize: "27px"
    fontWeight: 650
    lineHeight: 1.2
    letterSpacing: "-0.025em"
  editor-headline:
    fontFamily: "Segoe UI, sans-serif"
    fontSize: "25px"
    fontWeight: 600
    letterSpacing: "-0.025em"
  title:
    fontFamily: "Segoe UI, sans-serif"
    fontSize: "16px"
    fontWeight: 600
  body:
    fontFamily: "Segoe UI, sans-serif"
    fontSize: "14px"
    lineHeight: 1.5
  control:
    fontFamily: "Segoe UI, sans-serif"
    fontSize: "13px"
  label:
    fontFamily: "Segoe UI, sans-serif"
    fontSize: "12px"
rounded:
  acrylic-panel: "14px"
  acrylic-mark: "7px"
  control: "6px"
  status: "5px"
  popover: "8px"
  dialog: "12px"
spacing:
  compact: "8px"
  control-y: "9px"
  control-x: "14px"
  group: "16px"
  section: "24px"
  editor: "30px"
  workspace: "32px"
components:
  button-primary:
    backgroundColor: "{colors.crimson}"
    textColor: "{colors.light-button-ink}"
    rounded: "{rounded.control}"
    padding: "9px 14px"
    typography: "{typography.control}"
  button-secondary:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.crimson-text}"
    rounded: "{rounded.control}"
    padding: "9px 14px"
  button-primary-dark:
    backgroundColor: "{colors.dark-accent}"
    textColor: "{colors.dark-paper}"
    rounded: "{rounded.control}"
    padding: "9px 14px"
  field:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.crimson-text}"
    rounded: "{rounded.control}"
    padding: "9px"
  navigation-active:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.crimson}"
    rounded: "{rounded.control}"
    padding: "11px 10px"
---

# Design System: SmartOrganizer Flow

## Overview

Flow is the user-selected crimson interface for a desktop folder automation tool. Working screens use a persistent navigation region, a readable central workspace, and restrained crimson accents. The editor makes the actual mechanism visible through Watch, Match, and Act stages.

This document records the production implementation, not the alternative concept mockups. Component CSS and the Flow properties in src/style.css are its sources. Surface strategy remains in .impeccable/surfaces/flow.md.

Key characteristics are tinted neutral surfaces, flat separation, sentence case, explicit statuses, and visible actions. Segoe UI follows the Windows desktop setting without an ornamental display face.

## Colors

### Primary

Crimson is used for primary actions, selected navigation, matching controls, focus rings, and successful activity indicators. Crimson selected supplies a quieter highlighted background. Primary button ink changes with the installed theme.

### Neutral

Paper holds the content workspace; crimson surface holds navigation and secondary regions; crimson border separates rows and regions. Crimson text carries primary content and crimson muted carries descriptive copy, paths, and time labels.

The .dark class switches every Flow role to the corresponding dark primitive in the frontmatter: crimson-black content and navigation surfaces, pale foreground, muted crimson secondary text, and a lighter crimson action accent. Use role variables rather than hardcoded light colors when extending a screen. Legacy HSL properties remain for shared Radix/Tailwind controls; the Flow properties are the authority for the new shell and working screens.

**The Role Pair Rule.** Foreground and background must switch together. In particular, a light crimson button in dark mode takes dark ink.

## Typography

One workhorse UI family serves all roles. Dashboard and Statistics use the headline token, while Journal and the listener editor use the smaller editor headline. Section headings use the title role; descriptions and controls use the control role; paths, timestamps, and table headers use the label role. Journal event action metadata currently uses an isolated smaller size, recorded below as a noncanonical exception.

Headings use sentence case and slight negative tracking. Values in summary counts use tabular numerals; journal timestamps and statistical counts do likewise. Long file paths truncate in list rows and wrap in details. Text is not globally selection-disabled.

## Layout

The app fills the desktop window, with a titlebar above a flexible body. Navigation is 205px wide, 174px below 760px, and 72px when collapsed. Main content has a minimum width of zero and scrolls independently. Dashboard and Statistics use 32px desktop padding, 24px at widths below 1000px, and 18px below 650px. Journal uses 30px, then 20px below 760px and 14px below 450px.

At 520px and narrower, global rules move navigation above content, hide its brand/search/new-listener controls, and display navigation and settings controls in rows. The dashboard retains its own create action. Dashboard hides its activity column below 1000px and its rule count below 650px. Tables can scroll where their content requires it.

The editor has a header, independently scrolling working area, and save footer. Its desktop working area uses a flexible stage column and 210px summary column separated by a 30px gap. At 760px, the summary moves above the stages, the separator becomes horizontal, and padding becomes 20px. The three real stages use numbered circles and thin connecting lines. Radio options and fields wrap instead of forcing the viewport wider. At 520px the action choice grid reduces from three to two columns.

## Elevation & Depth

The production workspace is flat. Background tone and one-pixel separators distinguish navigation, tables, fields, and the editor summary. The editor explicitly removes inherited medium and extra-large shadows. Shared dialogs retain their inherited Radix overlay/elevation behavior; this does not make shadowed cards a workspace pattern.

Navigation width transitions over 180ms with ease-out when reduced motion is not requested. Global reduced-motion rules disable transitions and animations.

## Shapes

Controls and fields use gently rounded six-pixel corners; status controls use five-pixel corners. Sort popovers use eight-pixel corners; shared dialog radius is twelve pixels. Borders are one pixel. Step numbers and activity dots are circular. The sidebar mark is a small rounded crimson outline tilted slightly; it is not a new raster asset.

## Components

### Buttons

Primary actions use crimson fill in light mode and pale crimson in dark mode. Secondary buttons use the page surface and a thin border. Icon actions use muted ink and a selected-crimson background on hover. Keyboard focus is a two-pixel accent outline offset by three pixels. Disabled buttons use reduced opacity and the implementation's wait cursor; save displays a Saving state.

### Inputs / Fields

Fields use the page ground, primary text, thin role border, and control radius. Search labels gain the focus outline through focus-within. Native radio and checkbox accents and caret color use the primary role. Editor name and path inputs stay legible in both themes. Selection uses the selected-crimson fill with primary text. Scrollbar thumbs use the border role.

### Navigation

Links use actual router navigation and expose the current page. Active links use the page background and accent ink. In collapsed mode, labels disappear and accessible names and native titles remain. Settings and appearance are buttons; the enabled-listener count comes from the actual store.

### Listener table and activity

Listener titles open the editor. Explicit row buttons edit, delete, or enable/pause a listener. Status combines text with a dot, and drafts reflect incomplete listener data. Search and first-use states provide an actionable recovery. Recent activity and statistics are derived from actual stored logs; counts are not decorative sample claims.

### Listener editor

Watch, Match, and Act remain together in the working area. The right summary holds the editable name and delete action; the footer holds enabled state and explicit save. Save failure is announced; unsaved close and deletion use native confirmation dialogs. Matching supports Any and All. The production editor preserves existing monitoring, matching, and action capabilities.

### Journal

Search and native details-based sorting sit above real events. Each event opens a shared dialog showing listener and event details. Copy gives success or failure feedback. Empty and filtered-empty states differ. Below 760px the mescrimson moves to a second row; below 450px detail label/value pairs stack.

## Do's and Don'ts

- Do use Flow role variables for both light and dark appearances.
- Do keep status labels, counts, paths, and activity tied to actual application data.
- Do retain keyboard focus, semantic actions, and explicit save feedback.
- Do wrap long paths in detail views while keeping list rows scannable.
- Don't add mockup sample data to production screens.
- Don't introduce ornamental gradients, illustration panels, or shadowed metric cards into the flat working regions.
- Don't turn a route-specific layout into a universal requirement for every future screen.

## Acrylic reference update

The user supplied a dark Steam-style reference and requested an acrylic fluorescent background. `src/acrylic.css`, imported after base tokens, now governs the shell material and dark palette. Existing Flow structure and functionality remain.

Dark roles: ink #f6f1f3, muted #c6bfc8, base #171b24, surface #1c202a, border #42424e, warm action #ff9b70 with ink #25151a, selection #49313a. Shell uses rgba(17,21,29,.86), panel rgba(27,28,38,.68). Crimson and amber radial light fields are stationary behind the work areas. Work panels use 30–32px backdrop blur, 14px shell/panel corners and 8px primary controls. Native desktop wallpaper blur is not enabled. Unsupported backdrop-filter and reduced-transparency modes use solid surfaces. Light appearance retains crimson accents with translucent pale surfaces.

The sidebar has no background fill or backdrop filter in either theme or fallback mode. It exposes the shell light field directly; only individual search, selected-route and action controls have surfaces.

## Listener dialog and actions
The listener editor is bounded to 860px with 32px desktop side clearance, 40px vertical clearance, 16px corners, dimmed full-window backdrop and an offset soft shadow. Its header, scrollable single-column form, and fixed Cancel/Save footer form a clear dialog. At 600px it uses 12px side / 16px vertical clearance. Escape invokes the existing unsaved-changes flow.
Action rows use 10px corners, an operation select and a separately labelled destination field with Browse/Change. MOVE is initially selected but choosing a folder is independent. MOVE/COPY preserve their shared destination on type changes; rename uses a file save picker; destructive actions omit destination controls. Permanent delete carries an explicit warning. Picker cancellation preserves the current path.

Sidebar spacing refinement: expanded width 176px (160px at compact desktop widths), collapsed width 56px. Desktop left inset is 6px, right inset 10px, with 8px navigation control padding and 12px separation from the workspace. Active marker sits 5px left of its route control so it remains inside the window. Mobile top navigation retains its existing layout.
