using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[System.Serializable]
public class Token
{
    public string name;
    public int weapons;
    public int engineering;
    public int biotech;
    public int psychics;

    public static Token CreateFromJSON(string jsonString)
    {
        return JsonUtility.FromJson<Token>(jsonString);
    }
}
