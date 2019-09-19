import QtQuick 2.4
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0

Button {
    property string source
    height: 25
    width: height
    background: Image {
        id: background
        source: parent.source
        height: width
        scale: 0.9
        mipmap: true
    }
}