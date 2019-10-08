import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

// TODO: demagic with libherald import
// TODO: js for switching and choosing read receipts
ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string opName: "@unknown"
    property string opBody: ownedConversation.messageBodyById(op)
    property color opColor: "gray"
    property string authorName: ""

    spacing: 0

    Label {
        id: sender
        text: authorName === "" ? "" : "@" + authorName
        Layout.margins: authorName === "" ? 0 : QmlCfg.smallMargin
        Layout.bottomMargin: authorName === "" ? QmlCfg.smallMargin : QmlCfg.margin
        Layout.preferredHeight: authorName !== "" ? QmlCfg.margin : 0
        font.bold: true
        color: outbound ? "black" : "white"
    }

    Rectangle {
        id: replyWrapper
        height: reply.height
        color: opColor
        radius: 10
        Layout.margins: 5
        Layout.topMargin: 0
        Layout.minimumWidth: reply.width
        ColumnLayout {
            id: reply
            spacing: 1
            Label {
                id: opLabel
                text: opName
                Layout.margins: 5
                Layout.bottomMargin: 0
                Layout.preferredHeight: opName !== "" ? implicitHeight : 0
            }

            TextEdit {
                Layout.margins: 5
                text: opBody
                Layout.maximumWidth: Math.max(parent.maxWidth, 200)
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                color: outbound ? "black" : "white"
            }
        }
    }

    TextEdit {
        text: body
        Layout.leftMargin: 5
        Layout.rightMargin: 5
        Layout.maximumWidth: Math.max(parent.maxWidth, 200)
        Layout.alignment: Qt.AlignLeft
        selectByMouse: true
        selectByKeyboard: true
        readOnly: true
        color: outbound ? "black" : "white"
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    }

    RowLayout {
        Layout.margins: 5

        Label {
            font.pixelSize: QmlCfg.chatTextSize
            text: friendlyTimestamp
            id: timestamp
            color: outbound ? "black" : "white"
        }

        Item {
            Layout.fillWidth: true
        }

        Image {
            id: receipt
            source: receiptImage
            sourceSize: Qt.size(16, 16)
        }
    }
}
