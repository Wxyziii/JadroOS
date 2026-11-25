import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ApplicationWindow {
    id: root
    width: 660
    height: 360
    minimumWidth: 440
    minimumHeight: 300
    visible: true
    title: "JadroOS — Spotlight prototype"
    property string statusText: "Offline prototype — no provider calls"
    property var providers: ["General", "ChatGPT", "Claude", "Codex", "Claude Code", "Local AI"]

    function resetPanel() {
        tabs.currentIndex = 0
        mode.currentIndex = 0
        query.clear()
        statusText = "Offline prototype — no provider calls"
        query.forceActiveFocus()
    }
    function submitPreview() {
        if (!query.text.trim().length) {
            statusText = "Enter a query first."
            return
        }
        if (tabs.currentIndex === 0)
            statusText = "Planned route: local search (not implemented)"
        else if (tabs.currentIndex === 1 && mode.currentIndex === 0)
            statusText = "Planned route: official ChatGPT; manual handoff if needed"
        else if (tabs.currentIndex === 1)
            statusText = "Planned route: eligible native inference or explicit official UI selection; shared usage may apply"
        else
            statusText = "Planned route: " + providers[tabs.currentIndex] + " (not connected)"
    }

    Component.onCompleted: resetPanel()
    Shortcut { sequence: "Ctrl+Tab"; onActivated: tabs.currentIndex = (tabs.currentIndex + 1) % root.providers.length }
    Shortcut { sequence: "Esc"; onActivated: root.resetPanel() }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 14
        Label {
            text: "Spotlight"
            font.pixelSize: 24
            font.bold: true
        }
        TextField {
            id: query
            Layout.fillWidth: true
            placeholderText: tabs.currentIndex === 0 ? "Search apps, files and settings" : "Ask " + root.providers[tabs.currentIndex]
            Accessible.name: placeholderText
            onAccepted: root.submitPreview()
        }
        TabBar {
            id: tabs
            Layout.fillWidth: true
            onCurrentIndexChanged: {
                root.statusText = "Offline prototype — no provider calls"
                mode.currentIndex = 0
            }
            Repeater {
                model: root.providers
                TabButton { text: modelData; Accessible.name: modelData }
            }
        }
        ComboBox {
            id: mode
            visible: tabs.currentIndex === 1
            model: ["Instant", "Medium", "High", "Extra High"]
            Accessible.name: "ChatGPT reasoning mode"
        }
        Label {
            Layout.fillWidth: true
            wrapMode: Text.WordWrap
            text: tabs.currentIndex === 1 && mode.currentIndex === 0
                ? "Official Instant chat is the intended route; never silently use native agent quota."
                : "This prototype performs no search, authentication, or external requests."
        }
        Item { Layout.fillHeight: true }
        Label { Layout.fillWidth: true; wrapMode: Text.WordWrap; text: root.statusText; Accessible.name: text }
        RowLayout {
            Button { text: "Reset"; onClicked: root.resetPanel() }
            Item { Layout.fillWidth: true }
            Button { text: "Preview route"; onClicked: root.submitPreview() }
        }
    }
}
