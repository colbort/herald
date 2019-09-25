import QtQuick 2.13
import LibHerald 1.0
import QtGraphicalEffects 1.0
import QtQuick.Controls 2.13

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

/// --- displays a list of contacts
Row {
    id: wrapperRow
    property string displayName: ""
    property string pfpUrl: ""
    property int colorHash: 0
    property int shapeEnum: 0 /// { individual, group ... }
    property int size: 0 /// the size of the avatar, width and height
    property int shape: 0
    property bool labeled: true /// whether or not to show the name
    property bool isDefault: true /// should be isInverted

    spacing: QmlCfg.padding
    ///--- Circle with initial
    leftPadding: QmlCfg.margin
    anchors.verticalCenter: parent.verticalCenter

    Loader {
        width: size
        height: size
        sourceComponent: JS.avatarSource(displayName, pfpUrl, imageAvatar,
                                         initialAvatar)
    }

    Text {
        visible: labeled
        text: displayName
        font.bold: true
        anchors.verticalCenter: parent.verticalCenter
        //is white instead of palette maincolor bc shld be white regardless of theme
        color: if (!isDefault) {
                   "white"
               } else {
                   "black"
               }
    }

    ///--- potential avatar components
    /// NPB: looks very clunky and bad by default, choose fonts, finalize design, maybe don't do
    /// what every other chat app does for this? are there easier to track avatars out there?
    Component {
        id: initialAvatar
        Rectangle {
            id: avatarRect
            width: size
            height: size
            anchors.verticalCenter: parent.verticalCenter
            //is white instead of palette maincolor bc shld be white regardless of theme
            color: if (!isDefault) {
                       "white"
                   } else {
                       QmlCfg.avatarColors[colorHash]
                   }
            // Note:
            radius: shape
            ///---- initial
            Text {
                text: qsTr(displayName[0].toUpperCase())
                font.bold: true
                color: if (!isDefault) {
                           QmlCfg.avatarColors[colorHash]
                       } else {
                           "white"
                       }
                anchors.centerIn: parent
                font.pixelSize: size * 2 / 3
            }
        }
    }

    ///--- image compoenent
    Component {
        id: imageAvatar
        Item {
            Rectangle {
                color: QmlCfg.palette.mainColor
                width: size
                height: size
                radius: shape
                id: mask
            }
            Image {
                source: Utils.safeToQrcURI(pfpUrl)
                anchors.fill: mask
                layer.enabled: true
                layer.effect: OpacityMask {
                    maskSource: mask
                }
                clip: true
                mipmap: true
            }
        }
    }
}