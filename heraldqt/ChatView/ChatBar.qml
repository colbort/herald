import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

ToolBar {
    property alias chatBarAvatar: chatBarAvatar
    clip: true
    height: QmlCfg.toolbarHeight
    anchors {
        top: parent.top
        left: parent.left
        right: parent.right
    }


    Common.Avatar {

       Connections {
           target: sideBar.contactData
           /// this is what the code would call anyways
           onDataChanged : {
               chatBarAvatar.displayName = Utils.try_index_or(sideBar.contactUi.currentIndex,
                                                  sideBar.contactData.name,
                                                  sideBar.contactData.rowCount(),"")

                chatBarAvatar.colorHash = Utils.try_index_or(sideBar.contactUi.currentIndex,
                                               sideBar.contactData.color,
                                               sideBar.contactData.rowCount(), 0)

                  chatBarAvatar.pfpUrl = Utils.try_index_or(sideBar.contactUi.currentIndex,
                                             sideBar.contactData.profile_picture,
                                              sideBar.contactData.rowCount(),"")
           }
       }


     displayName : Utils.try_index_or(sideBar.contactUi.currentIndex,
                                        sideBar.contactData.name,
                                        sideBar.contactData.rowCount(),"")

      colorHash : Utils.try_index_or(sideBar.contactUi.currentIndex,
                                     sideBar.contactData.color,
                                     sideBar.contactData.rowCount(), 0)

        pfpUrl: Utils.try_index_or(sideBar.contactUi.currentIndex,
                                   sideBar.contactData.profile_picture,
                                    sideBar.contactData.rowCount(),"")
        id: chatBarAvatar

        anchors.centerIn: parent
        size: QmlCfg.toolbarHeight - QmlCfg.margin
    }

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        anchors.fill: parent
    }
}
