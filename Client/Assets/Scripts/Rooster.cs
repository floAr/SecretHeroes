using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Rooster : MonoBehaviour
{
    public List<Token> MyHeroes;

    [ContextMenu("Test Add Token")]
    public void AddToken()
    {
        int i = MyHeroes.Count;
        MyHeroes.Add(new Token() { biotech = i, engineering = i, psychics = i, weapons = i, name = "" });
    }

    private void Start()
    {
        MyHeroes.Clear();
    }

    internal void UpdateTokens(Token[] tokens)
    {
        foreach (var item in tokens)
        {
            var hero = MyHeroes.Find(t => t.id.Equals(item.id));
            if (hero == null)
            {
                MyHeroes.Add(item);
            }
            else
            {
                hero.weapons = item.weapons;
                hero.engineering = item.engineering;
                hero.biotech = item.biotech;
                hero.psychics = item.psychics;
            }
        }
        //MyHeroes.AddRange(tokens);

    }

    internal void UpdateToken(Token item)
    {
        var hero = MyHeroes.Find(t => t.id == item.id);
        if (hero == null)
        {
            MyHeroes.Add(item);
        }
        else
        {
            hero.weapons = item.weapons;
            hero.engineering = item.engineering;
            hero.biotech = item.biotech;
            hero.psychics = item.psychics;
        }
    }

    internal void RemoveToken(Token token)
    {
        Token t = MyHeroes.Find(t => t.id == token.id);

        if (t != null)
        {
            MyHeroes.Remove(t);
        }
    }

    internal Token FindToken(string id)
    {
        Token t = new Token();
        t = MyHeroes.Find(t => t.id == id);
        return t;
    }

}
