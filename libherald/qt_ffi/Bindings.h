/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Config;
class Conversation;
class Conversations;
class HeraldState;
class HeraldUtils;
class NetworkHandle;
class Users;

class Config : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint32 color READ color WRITE setColor NOTIFY colorChanged FINAL)
    Q_PROPERTY(quint32 colorscheme READ colorscheme WRITE setColorscheme NOTIFY colorschemeChanged FINAL)
    Q_PROPERTY(QString configId READ configId NOTIFY configIdChanged FINAL)
    Q_PROPERTY(QString displayName READ displayName NOTIFY displayNameChanged FINAL)
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged FINAL)
    Q_PROPERTY(QString profilePicture READ profilePicture WRITE setProfilePicture NOTIFY profilePictureChanged FINAL)
    explicit Config(bool owned, QObject *parent);
public:
    explicit Config(QObject *parent = nullptr);
    ~Config();
    quint32 color() const;
    void setColor(quint32 v);
    quint32 colorscheme() const;
    void setColorscheme(quint32 v);
    QString configId() const;
    QString displayName() const;
    QString name() const;
    void setName(const QString& v);
    QString profilePicture() const;
    void setProfilePicture(const QString& v);
Q_SIGNALS:
    void colorChanged();
    void colorschemeChanged();
    void configIdChanged();
    void displayNameChanged();
    void nameChanged();
    void profilePictureChanged();
};

class Conversation : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint32 color READ color WRITE setColor NOTIFY colorChanged FINAL)
    Q_PROPERTY(QByteArray conversationId READ conversationId WRITE setConversationId NOTIFY conversationIdChanged FINAL)
    Q_PROPERTY(bool muted READ muted WRITE setMuted NOTIFY mutedChanged FINAL)
    Q_PROPERTY(bool pairwise READ pairwise NOTIFY pairwiseChanged FINAL)
    Q_PROPERTY(QString picture READ picture WRITE setPicture NOTIFY pictureChanged FINAL)
    Q_PROPERTY(QString title READ title WRITE setTitle NOTIFY titleChanged FINAL)
    explicit Conversation(bool owned, QObject *parent);
public:
    explicit Conversation(QObject *parent = nullptr);
    ~Conversation();
    quint32 color() const;
    void setColor(quint32 v);
    QByteArray conversationId() const;
    void setConversationId(const QByteArray& v);
    bool muted() const;
    void setMuted(bool v);
    bool pairwise() const;
    QString picture() const;
    void setPicture(const QString& v);
    QString title() const;
    void setTitle(const QString& v);
    Q_INVOKABLE void clearConversationView();
    Q_INVOKABLE bool deleteConversation();
    Q_INVOKABLE bool deleteConversationById(const QByteArray& conversation_id);
    Q_INVOKABLE bool deleteMessage(quint64 row_index);
    Q_INVOKABLE QByteArray insertMessage(const QString& body);
    Q_INVOKABLE bool refresh();
    Q_INVOKABLE QByteArray reply(const QString& body, const QByteArray& op);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QString author(int row) const;
    Q_INVOKABLE QString body(int row) const;
    Q_INVOKABLE qint64 epochTimestampMs(int row) const;
    Q_INVOKABLE QByteArray messageId(int row) const;
    Q_INVOKABLE QByteArray op(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void colorChanged();
    void conversationIdChanged();
    void mutedChanged();
    void pairwiseChanged();
    void pictureChanged();
    void titleChanged();
};

class Conversations : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
    Q_PROPERTY(bool filterRegex READ filterRegex WRITE setFilterRegex NOTIFY filterRegexChanged FINAL)
    explicit Conversations(bool owned, QObject *parent);
public:
    explicit Conversations(QObject *parent = nullptr);
    ~Conversations();
    QString filter() const;
    void setFilter(const QString& v);
    bool filterRegex() const;
    void setFilterRegex(bool v);
    Q_INVOKABLE QByteArray addConversation();
    Q_INVOKABLE bool hardRefresh();
    Q_INVOKABLE bool removeConversation(quint64 row_index);
    Q_INVOKABLE bool toggleFilterRegex();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE quint32 color(int row) const;
    Q_INVOKABLE bool setColor(int row, quint32 value);
    Q_INVOKABLE QByteArray conversationId(int row) const;
    Q_INVOKABLE bool matched(int row) const;
    Q_INVOKABLE bool setMatched(int row, bool value);
    Q_INVOKABLE bool muted(int row) const;
    Q_INVOKABLE bool setMuted(int row, bool value);
    Q_INVOKABLE bool pairwise(int row) const;
    Q_INVOKABLE QString picture(int row) const;
    Q_INVOKABLE bool setPicture(int row, const QString& value);
    Q_INVOKABLE QString title(int row) const;
    Q_INVOKABLE bool setTitle(int row, const QString& value);

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void filterChanged();
    void filterRegexChanged();
};

class HeraldState : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(bool configInit READ configInit NOTIFY configInitChanged FINAL)
    explicit HeraldState(bool owned, QObject *parent);
public:
    explicit HeraldState(QObject *parent = nullptr);
    ~HeraldState();
    bool configInit() const;
    Q_INVOKABLE bool setConfigId(const QString& config_id);
Q_SIGNALS:
    void configInitChanged();
};

class HeraldUtils : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit HeraldUtils(bool owned, QObject *parent);
public:
    explicit HeraldUtils(QObject *parent = nullptr);
    ~HeraldUtils();
    Q_INVOKABLE double chatBubbleNaturalWidth(double chat_pane_width, double text_width) const;
    Q_INVOKABLE bool compareByteArray(const QByteArray& bs1, const QByteArray& bs2) const;
Q_SIGNALS:
};

class NetworkHandle : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(bool connectionPending READ connectionPending NOTIFY connectionPendingChanged FINAL)
    Q_PROPERTY(bool connectionUp READ connectionUp NOTIFY connectionUpChanged FINAL)
    Q_PROPERTY(bool newContact READ newContact NOTIFY newContactChanged FINAL)
    Q_PROPERTY(bool newConversation READ newConversation NOTIFY newConversationChanged FINAL)
    Q_PROPERTY(bool newMessage READ newMessage NOTIFY newMessageChanged FINAL)
    explicit NetworkHandle(bool owned, QObject *parent);
public:
    explicit NetworkHandle(QObject *parent = nullptr);
    ~NetworkHandle();
    bool connectionPending() const;
    bool connectionUp() const;
    bool newContact() const;
    bool newConversation() const;
    bool newMessage() const;
    Q_INVOKABLE bool registerDevice();
    Q_INVOKABLE bool requestMetaData(const QString& of);
    Q_INVOKABLE bool sendAddRequest(const QString& user_id, const QByteArray& conversation_id);
    Q_INVOKABLE bool sendMessage(const QString& message_body, const QByteArray& to, const QByteArray& msg_id);
Q_SIGNALS:
    void connectionPendingChanged();
    void connectionUpChanged();
    void newContactChanged();
    void newConversationChanged();
    void newMessageChanged();
};

class Users : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QByteArray conversationId READ conversationId WRITE setConversationId NOTIFY conversationIdChanged FINAL)
    Q_PROPERTY(QString filter READ filter WRITE setFilter NOTIFY filterChanged FINAL)
    Q_PROPERTY(bool filterRegex READ filterRegex WRITE setFilterRegex NOTIFY filterRegexChanged FINAL)
    explicit Users(bool owned, QObject *parent);
public:
    explicit Users(QObject *parent = nullptr);
    ~Users();
    QByteArray conversationId() const;
    void setConversationId(const QByteArray& v);
    QString filter() const;
    void setFilter(const QString& v);
    bool filterRegex() const;
    void setFilterRegex(bool v);
    Q_INVOKABLE QByteArray add(const QString& id);
    Q_INVOKABLE bool addToConversation(const QString& user_id, const QByteArray& conversation_id);
    Q_INVOKABLE bool addToConversationByIndex(quint64 row_index, const QByteArray& conversation_id);
    Q_INVOKABLE bool bulkAddToConversation(const QByteArray& user_id_array, const QByteArray& conversation_id);
    Q_INVOKABLE qint64 indexFromConversationId(const QByteArray& conversation_id) const;
    Q_INVOKABLE bool refresh();
    Q_INVOKABLE bool removeFromConversation(quint64 row_index, const QByteArray& conversation_id);
    Q_INVOKABLE bool toggleFilterRegex();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE quint32 color(int row) const;
    Q_INVOKABLE bool setColor(int row, quint32 value);
    Q_INVOKABLE QString displayName(int row) const;
    Q_INVOKABLE bool matched(int row) const;
    Q_INVOKABLE bool setMatched(int row, bool value);
    Q_INVOKABLE QString name(int row) const;
    Q_INVOKABLE bool setName(int row, const QString& value);
    Q_INVOKABLE QByteArray pairwiseConversationId(int row) const;
    Q_INVOKABLE QString profilePicture(int row) const;
    Q_INVOKABLE bool setProfilePicture(int row, const QString& value);
    Q_INVOKABLE quint8 status(int row) const;
    Q_INVOKABLE bool setStatus(int row, quint8 value);
    Q_INVOKABLE QString userId(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void conversationIdChanged();
    void filterChanged();
    void filterRegexChanged();
};
#endif // BINDINGS_H
