# bevy-inspector-ui

Bevy-ui inspector components inspired by `bevy-inspector-egui`

Reference design system [figma](https://www.figma.com/design/jLdiumully7s5wbCt5rcMb/Space-Editor?node-id=0-1&node-type=canvas&t=gojAPu20NbhH2vI9-0)

Bevy-inspector-ui is composed of 3 main crates `bevy-inspector-ui`, `bevy-inspector-ui-derive` and `bevy-widgets`. The first 2 are a version of `bevy-inspector-egui` using bevy_ui and the second is the supporting components.

## Bevy-widgets TODO:

- [] Text Input Field
    - [] Container label (~ hint text).
- [] Dropdown
    - [] states: default, hover, focus, selected and disabled
    - [] open and closed states
    - [] react to open an close
    - [] select element and send event on selection
- [] Avatar
    - [] support image
    - [] connection state
    - [] default avatar
- [] Alert Banner
    - [] Default
    - [] warning
    - [] Error
    - [] react on alert event sent
    - [] close on timer
- [] Title Bar
- [] Search bar
    - [] fuzzy behavior
    - [] filter all components in sibling list of texts/labels
- [] Information labels
    - [] small
    - [] medium
- [] Event dispatcher
- [] Change input field on validation error
    - [] error
    - [] warning
- [] Hierarchy Menu
    - [] Hierarchy tabs
    - [] Hierarchy submenu
- [] Embedded icons
- [] Color field with states
- [] Number field
    - [] states: default, hover, focus, selected and disabled
    - [] vector fields
    - [] vector fields states
- [] Checkboxes => bool
    - [] states: default, hover, focus, selected and disabled
    - [] label
    - [] small
    - [] medium
- [] Radio Button => Enum
    - [] states: default, hover, focus, selected and disabled
    - [] label
    - [] small
    - [] medium
- [] Slider
    - [] min max labels
    - [] current value
    - [] label
    - [] states: default, hover, focus, selected and disabled
- [] Console
    - [] react to logs and alert events
    - [] clear
    - [] filter for errors
    - [] filter for warnings
    - [] expand/collapse
    - [] search bar
- [] Accordion
    - [] open/closed
    - [] selected element
- [] Color Picker
- [] Text Labels
    - [] H1
    - [] H2
    - [] H3
    - [] H4
    - [] headline
    - [] body
    - [] subtitle
    - [] caption
    - [] footnote
