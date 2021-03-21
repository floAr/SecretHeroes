using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Rooster : MonoBehaviour
{
    public List<Token> MyHeroes;

    [ContextMenu("TST ADD")]
    public void AddToken()
    {
        int i = MyHeroes.Count;
        MyHeroes.Add(new Token() { biotech = i, engineering = i, psychics = i, weapons = i, name = "" });
    }

    //private void Start()
    //{
    //    for (int i = 0; i < 3; i++)
    //    {
    //        AddToken();
    //    }
    //}

    internal void UpdateTokens(Token[] tokens)
    {
        foreach (var item in tokens)
        {

            if(MyHeroes.Find(t => { return t.name.Equals(item.name); }) == null)
            {
                MyHeroes.Add(item);
            }
        }
    }
}
