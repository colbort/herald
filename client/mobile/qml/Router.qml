import QtQuick 2.14
import QtQuick.Controls 2.12
import LibHerald 1.0

Item {
    id: router
    property StackView stack: null
    property var searchView
    property var cvView
    property var activeChatView
    property var contactView
    signal convoRequest(var searchConversationId, var searchMsgId)
    signal messagePosRequested(var requestedMsgId)
    signal convoClicked(var searchConversationId)
    signal groupRequested(var groupId)

    Loader {
        active: searchView !== null && searchView !== undefined
        sourceComponent: Connections {
            target: router.searchView
            onMessageClicked: router.convoRequest(searchConversationId,
                                                  searchMsgId)
        }
    }

    Loader {
        active: contactView !== undefined
        sourceComponent: Connections {
            target: router.contactView
            onGroupClicked: {
                groupRequested(groupId)
            }
        }
    }

    Loader {
        active: activeChatView !== null
        sourceComponent: Connections {
            target: router.cvView
            onMessagePositionRequested: {
                messagePosRequested(requestMsgId)
            }
        }
    }

    Loader {
        active: searchView !== null && searchView !== undefined
        sourceComponent: Connections {
            target: router.searchView
            onConvoClicked: router.convoClicked(searchConversationId)
        }
    }
}
