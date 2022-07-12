import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Window {
    width: 640
    height: 480
    visible: true
    title: "QtNotes"

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent
        anchors.rightMargin: 20
        anchors.leftMargin: 20
        anchors.bottomMargin: 20
        anchors.topMargin: 20
        spacing: 6

        Text {
            id: title
            text: qsTr("QtNotes")
            font.pixelSize: 22
            wrapMode: Text.WrapAnywhere
            renderType: Text.NativeRendering
            Layout.fillWidth: true
        }

        Text {
            id: description
            text: qsTr("A cross platform note taking application that does not rely on web technologies")
            font.pixelSize: 14
            wrapMode: Text.WordWrap
            renderType: Text.NativeRendering
            Layout.fillWidth: true
        }

        TextEdit {
            id: noteEdit
            width: 80
            height: 20
            text: qsTr("Your note")
            font.pixelSize: 12
            wrapMode: Text.WordWrap
            renderType: Text.NativeRendering
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        Button {
            id: createNoteButton
            text: qsTr("Create note")
            padding: 8
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

            Connections {
                target: createNoteButton
                function onClicked() { console.log("Create note clicked") }
            }
        }
    }
}
