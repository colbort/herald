import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Entity" as Ent
import "qrc:/imports/js/utils.mjs" as Utils
import "../ChatView" as ChatView
import "../Common" as Common

// Layout & tap behavior for a conversation item in conversation list view
Rectangle {
    id: conversationItem

    property string itemTitle
    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0
    // path to the conversation's avatar image
    property string imageSource: ''
    property bool isGroup: false
    // whether this conversation is the "Note to Self" conversation
    property bool isNTS: false
    // TOOD(cmck) shouldn't need to pass this in
    property ConversationContent convoContent
    // true if the conversation contains no messages
    property bool isEmpty
    // most recent message content to display in this item
    property var lastMsgDigest
    property alias ownedCV: ownedChatView
    property alias tapEnabled: tapHandler.enabled

    height: CmnCfg.convoHeight

    // prevent animation spill over
    clip: true

    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }

    Common.PlatonicRectangle {
        id: convoRectangle
        boxTitle: itemTitle
        boxColor: conversationItem.colorCode
        picture: imageSource
        isGroupPicture: conversationItem.isGroup
        isMessageResult: false

        labelComponent: Ent.ConversationLabel {
            convoTitle: itemTitle
            lastMsgDigest: conversationItem.lastMsgDigest
            isEmpty: conversationItem.isEmpty
        }
    }

    // background item which gets manipulated
    // during the on tap animation
    Rectangle {
        id: splash
        width: 0
        height: width
        color: CmnCfg.palette.iconMatte
        opacity: 0
        radius: width
        transform: Translate {
            x: -splash.width / 2
            y: -splash.height / 2
        }
    }

    ParallelAnimation {
        id: splashAnim
        NumberAnimation {
            target: splash
            property: "width"
            duration: CmnCfg.units.longDuration
            easing.type: Easing.InOutQuad
            from: parent.width / 2
            to: parent.width * 2
        }
        NumberAnimation {
            target: splash
            property: "opacity"
            duration: CmnCfg.units.longDuration
            easing.type: Easing.InOutQuad
            from: 0.2
            to: 0
        }
        onRunningChanged: if (!running)
                              mainView.push(ownedChatView)
    }

    Component {
        id: ownedChatView
        ChatView.ChatViewMain {
            property string stateName: "chat"
            headerTitle: itemTitle
            convoItem: conversationItem.conversationData
            convContent: convoContent
        }
    }

    TapHandler {
        id: tapHandler
        onTapped: {
            splash.x = eventPoint.position.x
            splash.y = eventPoint.position.y
            // set the chat to the selected item
            splashAnim.running = true
            // callback implicity called at the end of the animation
        }
    }
}
