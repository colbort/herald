import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.mjs" as Utils
import "ChatTextAreaUtils.mjs" as CTUtils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
Rectangle {
    id: self
    property string messageText: ""
    //color of the bubble proper
    property color bubbleColor
    // the message options button shown on hover
    property alias messageOptionsButton: messageOptionsButton
    // a mouse area to handle hover events
    property alias chatBubbleHitBox: chatBubbleHitbox
    // the width the text sits at without wrapping
    readonly property int naturalTextWidth: bubbleText.width
    // a component to use if there is additional content to spawn inside the chat bubble
    property string additionalContent: ""
    // the args to pass into the content spawner
    property var contentArgs

    color: bubbleColor
    radius: QmlCfg.radius

    // NPB: this flickers a lot, pause on scroll also
    // handles chatbubble hovering
    MouseArea {
        propagateComposedEvents: true
        id: chatBubbleHitbox
        hoverEnabled: true
        width: parent.width + 50

        onEntered: { messageOptionsButton.visible = !messageOptionsButton.visible
        replyButton.visible = !replyButton.visible}
        onExited: { messageOptionsButton.visible = !messageOptionsButton.visible
            replyButton.visible = !replyButton.visible}

        anchors {
            // Ternary is okay, types are enforced, cases are explicit.
            left: !outbound ? parent.left : undefined
            right: outbound ? parent.right : undefined
            bottom: parent.bottom
            top: parent.top
        }

        Common.ButtonForm {
            id: messageOptionsButton
            visible: false
            anchors {
                // Ternary is okay, types are enforced, cases are explicit.
                left: outbound ? parent.left : undefined
                right: !outbound ? parent.right : undefined
                margins: QmlCfg.margin
                verticalCenter: chatBubbleHitbox.verticalCenter
            }
            source: "qrc:/options-icon.svg"
            z: 10

            onClicked: {
                CTUtils.activateReplyPopup()
                print("kaavya! put some business logic here.")
            }
        }

        Common.ButtonForm {
            id: replyButton
            visible: false
            anchors {
                // Ternary is okay, types are enforced, cases are explicit.
                right: outbound ? messageOptionsButton.left : undefined
                left: !outbound ? messageOptionsButton.right : undefined
                margins: QmlCfg.margin
                verticalCenter: chatBubbleHitbox.verticalCenter
            }
            source: "qrc:/reply.png"
            z: 10

            onClicked: {
                CTUtils.activateReplyPopup()
                print("kaavya! put some business logic here.")
            }
        }
    }



    // NPB find a better generic way to spawn items inside of chat bubbles, states and loaders
    Component.onCompleted: {
        contentArgs.uiContainer = bubbleText
        attachmentLoader.setSource(additionalContent, contentArgs)
    }

    width: bubble.width
    height: bubble.height

    // column that loads each chat bubble + additional content
    Column {
        id: bubble
        padding: QmlCfg.smallPadding

        /// NBP: find a better way to generically load content
        Loader {
            id: attachmentLoader
            source: additionalContent
        }

        Common.CorrectText {
            id: bubbleText
            text: messageText
            width: heraldUtils.chatBubbleNaturalWidth(chatPane.width,
                                                      correctWidth)
            Layout.alignment: Qt.AlignLeft
            wrapMode: TextEdit.Wrap
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
            color: outbound ? "black" : "white"
        }

        Label {
            id: timeStamp
            color: outbound ? QmlCfg.palette.secondaryTextColor : Qt.lighter(
                                  QmlCfg.palette.secondaryTextColor, 1.5)
            text: Utils.friendlyTimestamp(epochTimestampMs)
            font.pointSize: QmlCfg.chatTextSize
        }
    }
}
