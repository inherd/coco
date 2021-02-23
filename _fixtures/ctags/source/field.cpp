// based on https://stackoverflow.com/questions/38217459/accessing-list-of-fields-and-types-in-a-class-in-c
#include <iostream>
#include <map>

using namespace std;

//the "Field" interface
class IFieldOrm
{
public:
    virtual ~IFieldOrm() {}

    virtual void save() = 0;
    virtual void migrate() = 0;
};

//your base class
class BaseOrm
{
public:
    virtual ~BaseOrm();

    virtual void save();
    virtual void migrate();

protected:
    map<string, IFieldOrm*> m_fields; //prefer a smart pointer if you don't want to mess with raw pointer
};

//base class implementation
void BaseOrm::save()
{
    for(auto& f : m_fields)
        f.second->save();
}

void BaseOrm::migrate()
{
    for(auto& f : m_fields)
        f.second->migrate();
}

//don't forget to free your "fields" pointers if you have raw pointers
BaseOrm::~BaseOrm()
{
    for(auto& f : m_fields)
        delete f.second;
}


//then implement your basic types
//(like string, int, ..., whatever type you want to store in your database)
class StringFieldOrm : public IFieldOrm
{
public:
    StringFieldOrm(const string& value) : m_value(value) {}

    virtual void save();
    virtual void migrate();

private:
    string m_value;
};

void StringFieldOrm::save()
{
    cout << "Save value " << m_value << endl;
    //save stuff...
}

void StringFieldOrm::migrate()
{
    cout << "Migrate value " << m_value << endl;
    //migrate stuff...
}

class IntFieldOrm : public IFieldOrm
{
public:
    IntFieldOrm(int& value) : m_value(value) {}

    virtual void save();
    virtual void migrate();

private:
    int m_value;
};

void IntFieldOrm::save()
{
    cout << "Save value " << m_value << endl;
    //save stuff...
}

void IntFieldOrm::migrate()
{
    cout << "Migrate value " << m_value << endl;
    //migrate stuff
}



//and finally implement your final class
//note that this object can be "dynamically extended" by inserting new fields,
//you may want to prevent that and I can think of a solution if you want to
class UserProfile: public BaseOrm
{
public:
    UserProfile(const string& username, const string& email, int age);
};

UserProfile::UserProfile(const string& username, const string& email, int age)
{
    m_fields["username"] = new StringFieldOrm(username);
    m_fields["email"] = new StringFieldOrm(email);
    m_fields["age"] = new IntFieldOrm(age);
}

int main(int argc, char* argv[])
{
    UserProfile user = UserProfile("Batman", "bw@batmail.com", 30);

    user.save();

    return 0;
}
