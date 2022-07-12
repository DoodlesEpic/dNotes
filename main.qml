import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Window {
    width: 640
    height: 480
    visible: true
    title: "QtNotes"

    ColumnLayout {
        id: column
        anchors.fill: parent
        anchors.rightMargin: 20
        anchors.leftMargin: 20
        anchors.bottomMargin: 20
        anchors.topMargin: 20
        spacing: 6

        Text {
            id: text1
            text: qsTr("QtNotes")
            font.pixelSize: 22
            renderType: Text.NativeRendering
            Layout.fillWidth: true
        }

        Text {
            id: text2
            text: qsTr("A cross platform note taking application that does not rely on web technologies")
            font.pixelSize: 14
            renderType: Text.NativeRendering
            Layout.fillWidth: true
        }

        TextEdit {
            id: textEdit
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
            id: button
            text: qsTr("Create note")
            padding: 8
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
        }
    }
}
