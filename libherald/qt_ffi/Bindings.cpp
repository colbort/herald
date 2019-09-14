/* generated by rust_qt_binding_generator */
#include "Bindings.h"

namespace {

    struct option_quintptr {
    public:
        quintptr value;
        bool some;
        operator QVariant() const {
            if (some) {
                return QVariant::fromValue(value);
            }
            return QVariant();
        }
    };
    static_assert(std::is_pod<option_quintptr>::value, "option_quintptr must be a POD type.");

    typedef void (*qstring_set)(QString* val, const char* utf8, int nbytes);
    void set_qstring(QString* val, const char* utf8, int nbytes) {
        *val = QString::fromUtf8(utf8, nbytes);
    }

    typedef void (*qbytearray_set)(QByteArray* val, const char* bytes, int nbytes);
    void set_qbytearray(QByteArray* v, const char* bytes, int nbytes) {
        if (v->isNull() && nbytes == 0) {
            *v = QByteArray(bytes, nbytes);
        } else {
            v->truncate(0);
            v->append(bytes, nbytes);
        }
    }

    struct qmodelindex_t {
        int row;
        quintptr id;
    };
    inline QVariant cleanNullQVariant(const QVariant& v) {
        return (v.isNull()) ?QVariant() :v;
    }
    inline void configColorChanged(Config* o)
    {
        Q_EMIT o->colorChanged();
    }
    inline void configColorschemeChanged(Config* o)
    {
        Q_EMIT o->colorschemeChanged();
    }
    inline void configConfigIdChanged(Config* o)
    {
        Q_EMIT o->configIdChanged();
    }
    inline void configNameChanged(Config* o)
    {
        Q_EMIT o->nameChanged();
    }
    inline void configProfilePictureChanged(Config* o)
    {
        Q_EMIT o->profilePictureChanged();
    }
    inline void contactsFilterChanged(Contacts* o)
    {
        Q_EMIT o->filterChanged();
    }
    inline void contactsFilterRegexChanged(Contacts* o)
    {
        Q_EMIT o->filterRegexChanged();
    }
    inline void heraldStateConfigInitChanged(HeraldState* o)
    {
        Q_EMIT o->configInitChanged();
    }
    inline void messagesConversationIdChanged(Messages* o)
    {
        Q_EMIT o->conversationIdChanged();
    }
    inline void networkHandleConnectionPendingChanged(NetworkHandle* o)
    {
        Q_EMIT o->connectionPendingChanged();
    }
    inline void networkHandleConnectionUpChanged(NetworkHandle* o)
    {
        Q_EMIT o->connectionUpChanged();
    }
    inline void networkHandleNewMessageChanged(NetworkHandle* o)
    {
        Q_EMIT o->newMessageChanged();
    }
}
extern "C" {
    Config::Private* config_new(Config*, void (*)(Config*), void (*)(Config*), void (*)(Config*), void (*)(Config*), void (*)(Config*));
    void config_free(Config::Private*);
    quint32 config_color_get(const Config::Private*);
    void config_color_set(Config::Private*, quint32);
    quint32 config_colorscheme_get(const Config::Private*);
    void config_colorscheme_set(Config::Private*, quint32);
    void config_config_id_get(const Config::Private*, QString*, qstring_set);
    void config_name_get(const Config::Private*, QString*, qstring_set);
    void config_name_set(Config::Private*, const ushort *str, int len);
    void config_name_set_none(Config::Private*);
    void config_profile_picture_get(const Config::Private*, QString*, qstring_set);
    void config_profile_picture_set(Config::Private*, const ushort *str, int len);
    void config_profile_picture_set_none(Config::Private*);
};

extern "C" {
    quint32 contacts_data_color(const Contacts::Private*, int);
    bool contacts_set_data_color(Contacts::Private*, int, quint32);
    void contacts_data_contact_id(const Contacts::Private*, int, QString*, qstring_set);
    bool contacts_data_matched(const Contacts::Private*, int);
    bool contacts_set_data_matched(Contacts::Private*, int, bool);
    void contacts_data_name(const Contacts::Private*, int, QString*, qstring_set);
    bool contacts_set_data_name(Contacts::Private*, int, const ushort* s, int len);
    bool contacts_set_data_name_none(Contacts::Private*, int);
    void contacts_data_pairwise_conversation_id(const Contacts::Private*, int, QByteArray*, qbytearray_set);
    void contacts_data_profile_picture(const Contacts::Private*, int, QString*, qstring_set);
    bool contacts_set_data_profile_picture(Contacts::Private*, int, const ushort* s, int len);
    bool contacts_set_data_profile_picture_none(Contacts::Private*, int);
    quint8 contacts_data_status(const Contacts::Private*, int);
    bool contacts_set_data_status(Contacts::Private*, int, quint8);
    void contacts_sort(Contacts::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int contacts_row_count(const Contacts::Private*);
    bool contacts_insert_rows(Contacts::Private*, int, int);
    bool contacts_remove_rows(Contacts::Private*, int, int);
    bool contacts_can_fetch_more(const Contacts::Private*);
    void contacts_fetch_more(Contacts::Private*);
}
int Contacts::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Contacts::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Contacts::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : contacts_row_count(m_d);
}

bool Contacts::insertRows(int row, int count, const QModelIndex &)
{
    return contacts_insert_rows(m_d, row, count);
}

bool Contacts::removeRows(int row, int count, const QModelIndex &)
{
    return contacts_remove_rows(m_d, row, count);
}

QModelIndex Contacts::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Contacts::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Contacts::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : contacts_can_fetch_more(m_d);
}

void Contacts::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        contacts_fetch_more(m_d);
    }
}
void Contacts::updatePersistentIndexes() {}

void Contacts::sort(int column, Qt::SortOrder order)
{
    contacts_sort(m_d, column, order);
}
Qt::ItemFlags Contacts::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    if (i.column() == 0) {
        flags |= Qt::ItemIsEditable;
    }
    return flags;
}

quint32 Contacts::color(int row) const
{
    return contacts_data_color(m_d, row);
}

bool Contacts::setColor(int row, quint32 value)
{
    bool set = false;
    set = contacts_set_data_color(m_d, row, value);
    if (set) {
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }
    return set;
}

QString Contacts::contactId(int row) const
{
    QString s;
    contacts_data_contact_id(m_d, row, &s, set_qstring);
    return s;
}

bool Contacts::matched(int row) const
{
    return contacts_data_matched(m_d, row);
}

bool Contacts::setMatched(int row, bool value)
{
    bool set = false;
    set = contacts_set_data_matched(m_d, row, value);
    if (set) {
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }
    return set;
}

QString Contacts::name(int row) const
{
    QString s;
    contacts_data_name(m_d, row, &s, set_qstring);
    return s;
}

bool Contacts::setName(int row, const QString& value)
{
    bool set = false;
    if (value.isNull()) {
        set = contacts_set_data_name_none(m_d, row);
    } else {
    set = contacts_set_data_name(m_d, row, value.utf16(), value.length());
    }
    if (set) {
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }
    return set;
}

QByteArray Contacts::pairwiseConversationId(int row) const
{
    QByteArray b;
    contacts_data_pairwise_conversation_id(m_d, row, &b, set_qbytearray);
    return b;
}

QString Contacts::profilePicture(int row) const
{
    QString s;
    contacts_data_profile_picture(m_d, row, &s, set_qstring);
    return s;
}

bool Contacts::setProfilePicture(int row, const QString& value)
{
    bool set = false;
    if (value.isNull()) {
        set = contacts_set_data_profile_picture_none(m_d, row);
    } else {
    set = contacts_set_data_profile_picture(m_d, row, value.utf16(), value.length());
    }
    if (set) {
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }
    return set;
}

quint8 Contacts::status(int row) const
{
    return contacts_data_status(m_d, row);
}

bool Contacts::setStatus(int row, quint8 value)
{
    bool set = false;
    set = contacts_set_data_status(m_d, row, value);
    if (set) {
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }
    return set;
}

QVariant Contacts::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(color(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(contactId(index.row()));
        case Qt::UserRole + 2:
            return QVariant::fromValue(matched(index.row()));
        case Qt::UserRole + 3:
            return cleanNullQVariant(QVariant::fromValue(name(index.row())));
        case Qt::UserRole + 4:
            return QVariant::fromValue(pairwiseConversationId(index.row()));
        case Qt::UserRole + 5:
            return cleanNullQVariant(QVariant::fromValue(profilePicture(index.row())));
        case Qt::UserRole + 6:
            return QVariant::fromValue(status(index.row()));
        }
        break;
    }
    return QVariant();
}

int Contacts::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Contacts::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "color");
    names.insert(Qt::UserRole + 1, "contactId");
    names.insert(Qt::UserRole + 2, "matched");
    names.insert(Qt::UserRole + 3, "name");
    names.insert(Qt::UserRole + 4, "pairwiseConversationId");
    names.insert(Qt::UserRole + 5, "profilePicture");
    names.insert(Qt::UserRole + 6, "status");
    return names;
}
QVariant Contacts::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Contacts::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

bool Contacts::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (index.column() == 0) {
        if (role == Qt::UserRole + 0) {
            if (value.canConvert(qMetaTypeId<quint32>())) {
                return setColor(index.row(), value.value<quint32>());
            }
        }
        if (role == Qt::UserRole + 2) {
            if (value.canConvert(qMetaTypeId<bool>())) {
                return setMatched(index.row(), value.value<bool>());
            }
        }
        if (role == Qt::UserRole + 3) {
            if (!value.isValid() || value.isNull() ||value.canConvert(qMetaTypeId<QString>())) {
                return setName(index.row(), value.value<QString>());
            }
        }
        if (role == Qt::UserRole + 5) {
            if (!value.isValid() || value.isNull() ||value.canConvert(qMetaTypeId<QString>())) {
                return setProfilePicture(index.row(), value.value<QString>());
            }
        }
        if (role == Qt::UserRole + 6) {
            if (value.canConvert(qMetaTypeId<quint8>())) {
                return setStatus(index.row(), value.value<quint8>());
            }
        }
    }
    return false;
}

extern "C" {
    Contacts::Private* contacts_new(Contacts*, void (*)(Contacts*), void (*)(Contacts*),
        void (*)(const Contacts*),
        void (*)(Contacts*),
        void (*)(Contacts*),
        void (*)(Contacts*, quintptr, quintptr),
        void (*)(Contacts*),
        void (*)(Contacts*),
        void (*)(Contacts*, int, int),
        void (*)(Contacts*),
        void (*)(Contacts*, int, int, int),
        void (*)(Contacts*),
        void (*)(Contacts*, int, int),
        void (*)(Contacts*));
    void contacts_free(Contacts::Private*);
    void contacts_filter_get(const Contacts::Private*, QString*, qstring_set);
    void contacts_filter_set(Contacts::Private*, const ushort *str, int len);
    bool contacts_filter_regex_get(const Contacts::Private*);
    void contacts_filter_regex_set(Contacts::Private*, bool);
    void contacts_add(Contacts::Private*, const ushort*, int, QByteArray*, qbytearray_set);
    qint64 contacts_index_from_conversation_id(const Contacts::Private*, const char*, int);
    bool contacts_toggle_filter_regex(Contacts::Private*);
};

extern "C" {
    HeraldState::Private* herald_state_new(HeraldState*, void (*)(HeraldState*));
    void herald_state_free(HeraldState::Private*);
    bool herald_state_config_init_get(const HeraldState::Private*);
    bool herald_state_set_config_id(HeraldState::Private*, const ushort*, int);
};

extern "C" {
    void messages_data_author(const Messages::Private*, int, QString*, qstring_set);
    void messages_data_body(const Messages::Private*, int, QString*, qstring_set);
    qint64 messages_data_epoch_timestamp_ms(const Messages::Private*, int);
    void messages_data_message_id(const Messages::Private*, int, QByteArray*, qbytearray_set);
    void messages_data_op(const Messages::Private*, int, QByteArray*, qbytearray_set);
    void messages_sort(Messages::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int messages_row_count(const Messages::Private*);
    bool messages_insert_rows(Messages::Private*, int, int);
    bool messages_remove_rows(Messages::Private*, int, int);
    bool messages_can_fetch_more(const Messages::Private*);
    void messages_fetch_more(Messages::Private*);
}
int Messages::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Messages::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Messages::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : messages_row_count(m_d);
}

bool Messages::insertRows(int row, int count, const QModelIndex &)
{
    return messages_insert_rows(m_d, row, count);
}

bool Messages::removeRows(int row, int count, const QModelIndex &)
{
    return messages_remove_rows(m_d, row, count);
}

QModelIndex Messages::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Messages::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Messages::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : messages_can_fetch_more(m_d);
}

void Messages::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        messages_fetch_more(m_d);
    }
}
void Messages::updatePersistentIndexes() {}

void Messages::sort(int column, Qt::SortOrder order)
{
    messages_sort(m_d, column, order);
}
Qt::ItemFlags Messages::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    return flags;
}

QString Messages::author(int row) const
{
    QString s;
    messages_data_author(m_d, row, &s, set_qstring);
    return s;
}

QString Messages::body(int row) const
{
    QString s;
    messages_data_body(m_d, row, &s, set_qstring);
    return s;
}

qint64 Messages::epochTimestampMs(int row) const
{
    return messages_data_epoch_timestamp_ms(m_d, row);
}

QByteArray Messages::messageId(int row) const
{
    QByteArray b;
    messages_data_message_id(m_d, row, &b, set_qbytearray);
    return b;
}

QByteArray Messages::op(int row) const
{
    QByteArray b;
    messages_data_op(m_d, row, &b, set_qbytearray);
    return b;
}

QVariant Messages::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(author(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(body(index.row()));
        case Qt::UserRole + 2:
            return QVariant::fromValue(epochTimestampMs(index.row()));
        case Qt::UserRole + 3:
            return QVariant::fromValue(messageId(index.row()));
        case Qt::UserRole + 4:
            return cleanNullQVariant(QVariant::fromValue(op(index.row())));
        }
        break;
    }
    return QVariant();
}

int Messages::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Messages::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "author");
    names.insert(Qt::UserRole + 1, "body");
    names.insert(Qt::UserRole + 2, "epochTimestampMs");
    names.insert(Qt::UserRole + 3, "messageId");
    names.insert(Qt::UserRole + 4, "op");
    return names;
}
QVariant Messages::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Messages::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

extern "C" {
    Messages::Private* messages_new(Messages*, void (*)(Messages*),
        void (*)(const Messages*),
        void (*)(Messages*),
        void (*)(Messages*),
        void (*)(Messages*, quintptr, quintptr),
        void (*)(Messages*),
        void (*)(Messages*),
        void (*)(Messages*, int, int),
        void (*)(Messages*),
        void (*)(Messages*, int, int, int),
        void (*)(Messages*),
        void (*)(Messages*, int, int),
        void (*)(Messages*));
    void messages_free(Messages::Private*);
    void messages_conversation_id_get(const Messages::Private*, QByteArray*, qbytearray_set);
    void messages_conversation_id_set(Messages::Private*, const char* bytes, int len);
    void messages_conversation_id_set_none(Messages::Private*);
    void messages_clear_conversation_view(Messages::Private*);
    bool messages_delete_conversation_by_id(Messages::Private*, const char*, int);
    bool messages_delete_message(Messages::Private*, quint64);
    bool messages_delete_conversation(Messages::Private*);
    void messages_insert_message(Messages::Private*, const ushort*, int, QByteArray*, qbytearray_set);
    void messages_reply(Messages::Private*, const ushort*, int, const char*, int, QByteArray*, qbytearray_set);
};

extern "C" {
    NetworkHandle::Private* network_handle_new(NetworkHandle*, void (*)(NetworkHandle*), void (*)(NetworkHandle*), void (*)(NetworkHandle*));
    void network_handle_free(NetworkHandle::Private*);
    bool network_handle_connection_pending_get(const NetworkHandle::Private*);
    bool network_handle_connection_up_get(const NetworkHandle::Private*);
    bool network_handle_new_message_get(const NetworkHandle::Private*);
    bool network_handle_register_device(NetworkHandle::Private*);
    bool network_handle_request_meta_data(NetworkHandle::Private*, const ushort*, int);
    bool network_handle_send_add_request(NetworkHandle::Private*, const ushort*, int, const char*, int);
    bool network_handle_send_message(NetworkHandle::Private*, const ushort*, int, const char*, int, const char*, int);
};

Config::Config(bool /*owned*/, QObject *parent):
    QObject(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
}

Config::Config(QObject *parent):
    QObject(parent),
    m_d(config_new(this,
        configColorChanged,
        configColorschemeChanged,
        configConfigIdChanged,
        configNameChanged,
        configProfilePictureChanged)),
    m_ownsPrivate(true)
{
}

Config::~Config() {
    if (m_ownsPrivate) {
        config_free(m_d);
    }
}
quint32 Config::color() const
{
    return config_color_get(m_d);
}
void Config::setColor(quint32 v) {
    config_color_set(m_d, v);
}
quint32 Config::colorscheme() const
{
    return config_colorscheme_get(m_d);
}
void Config::setColorscheme(quint32 v) {
    config_colorscheme_set(m_d, v);
}
QString Config::configId() const
{
    QString v;
    config_config_id_get(m_d, &v, set_qstring);
    return v;
}
QString Config::name() const
{
    QString v;
    config_name_get(m_d, &v, set_qstring);
    return v;
}
void Config::setName(const QString& v) {
    if (v.isNull()) {
        config_name_set_none(m_d);
    } else {
    config_name_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());
    }
}
QString Config::profilePicture() const
{
    QString v;
    config_profile_picture_get(m_d, &v, set_qstring);
    return v;
}
void Config::setProfilePicture(const QString& v) {
    if (v.isNull()) {
        config_profile_picture_set_none(m_d);
    } else {
    config_profile_picture_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());
    }
}
Contacts::Contacts(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Contacts::Contacts(QObject *parent):
    QAbstractItemModel(parent),
    m_d(contacts_new(this,
        contactsFilterChanged,
        contactsFilterRegexChanged,
        [](const Contacts* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Contacts* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Contacts* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Contacts* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Contacts* o) {
            o->beginResetModel();
        },
        [](Contacts* o) {
            o->endResetModel();
        },
        [](Contacts* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Contacts* o) {
            o->endInsertRows();
        },
        [](Contacts* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Contacts* o) {
            o->endMoveRows();
        },
        [](Contacts* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Contacts* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Contacts::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Contacts::~Contacts() {
    if (m_ownsPrivate) {
        contacts_free(m_d);
    }
}
void Contacts::initHeaderData() {
}
QString Contacts::filter() const
{
    QString v;
    contacts_filter_get(m_d, &v, set_qstring);
    return v;
}
void Contacts::setFilter(const QString& v) {
    contacts_filter_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());
}
bool Contacts::filterRegex() const
{
    return contacts_filter_regex_get(m_d);
}
void Contacts::setFilterRegex(bool v) {
    contacts_filter_regex_set(m_d, v);
}
QByteArray Contacts::add(const QString& id)
{
    QByteArray s;
    contacts_add(m_d, id.utf16(), id.size(), &s, set_qbytearray);
    return s;
}
qint64 Contacts::indexFromConversationId(const QByteArray& conversation_id) const
{
    return contacts_index_from_conversation_id(m_d, conversation_id.data(), conversation_id.size());
}
bool Contacts::toggleFilterRegex()
{
    return contacts_toggle_filter_regex(m_d);
}
HeraldState::HeraldState(bool /*owned*/, QObject *parent):
    QObject(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
}

HeraldState::HeraldState(QObject *parent):
    QObject(parent),
    m_d(herald_state_new(this,
        heraldStateConfigInitChanged)),
    m_ownsPrivate(true)
{
}

HeraldState::~HeraldState() {
    if (m_ownsPrivate) {
        herald_state_free(m_d);
    }
}
bool HeraldState::configInit() const
{
    return herald_state_config_init_get(m_d);
}
bool HeraldState::setConfigId(const QString& config_id)
{
    return herald_state_set_config_id(m_d, config_id.utf16(), config_id.size());
}
Messages::Messages(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Messages::Messages(QObject *parent):
    QAbstractItemModel(parent),
    m_d(messages_new(this,
        messagesConversationIdChanged,
        [](const Messages* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Messages* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Messages* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Messages* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Messages* o) {
            o->beginResetModel();
        },
        [](Messages* o) {
            o->endResetModel();
        },
        [](Messages* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Messages* o) {
            o->endInsertRows();
        },
        [](Messages* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Messages* o) {
            o->endMoveRows();
        },
        [](Messages* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Messages* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Messages::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Messages::~Messages() {
    if (m_ownsPrivate) {
        messages_free(m_d);
    }
}
void Messages::initHeaderData() {
}
QByteArray Messages::conversationId() const
{
    QByteArray v;
    messages_conversation_id_get(m_d, &v, set_qbytearray);
    return v;
}
void Messages::setConversationId(const QByteArray& v) {
    if (v.isNull()) {
        messages_conversation_id_set_none(m_d);
    } else {
    messages_conversation_id_set(m_d, v.data(), v.size());
    }
}
void Messages::clearConversationView()
{
    return messages_clear_conversation_view(m_d);
}
bool Messages::deleteConversationById(const QByteArray& conversation_id)
{
    return messages_delete_conversation_by_id(m_d, conversation_id.data(), conversation_id.size());
}
bool Messages::deleteMessage(quint64 row_index)
{
    return messages_delete_message(m_d, row_index);
}
bool Messages::delete_conversation()
{
    return messages_delete_conversation(m_d);
}
QByteArray Messages::insertMessage(const QString& body)
{
    QByteArray s;
    messages_insert_message(m_d, body.utf16(), body.size(), &s, set_qbytearray);
    return s;
}
QByteArray Messages::reply(const QString& body, const QByteArray& op)
{
    QByteArray s;
    messages_reply(m_d, body.utf16(), body.size(), op.data(), op.size(), &s, set_qbytearray);
    return s;
}
NetworkHandle::NetworkHandle(bool /*owned*/, QObject *parent):
    QObject(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
}

NetworkHandle::NetworkHandle(QObject *parent):
    QObject(parent),
    m_d(network_handle_new(this,
        networkHandleConnectionPendingChanged,
        networkHandleConnectionUpChanged,
        networkHandleNewMessageChanged)),
    m_ownsPrivate(true)
{
}

NetworkHandle::~NetworkHandle() {
    if (m_ownsPrivate) {
        network_handle_free(m_d);
    }
}
bool NetworkHandle::connectionPending() const
{
    return network_handle_connection_pending_get(m_d);
}
bool NetworkHandle::connectionUp() const
{
    return network_handle_connection_up_get(m_d);
}
bool NetworkHandle::newMessage() const
{
    return network_handle_new_message_get(m_d);
}
bool NetworkHandle::registerDevice()
{
    return network_handle_register_device(m_d);
}
bool NetworkHandle::requestMetaData(const QString& of)
{
    return network_handle_request_meta_data(m_d, of.utf16(), of.size());
}
bool NetworkHandle::sendAddRequest(const QString& user_id, const QByteArray& conversation_id)
{
    return network_handle_send_add_request(m_d, user_id.utf16(), user_id.size(), conversation_id.data(), conversation_id.size());
}
bool NetworkHandle::sendMessage(const QString& message_body, const QByteArray& to, const QByteArray& msg_id)
{
    return network_handle_send_message(m_d, message_body.utf16(), message_body.size(), to.data(), to.size(), msg_id.data(), msg_id.size());
}
