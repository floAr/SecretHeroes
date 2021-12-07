using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[System.Serializable]
public class Token
{
    public string id;
    public string name;
    public int weapons;
    public int engineering;
    public int biotech;
    public int psychics;


    public int base_weapons;
    public int base_engineering;
    public int base_biotech;
    public int base_psychics;

    public static Token CreateFromJSON(string jsonString)
    {
        return JsonUtility.FromJson<Token>(jsonString);
    }

    public static Token Random()
    {
        return new Token()
        {
            id = Mathf.FloorToInt(UnityEngine.Random.Range(0, 1000)).ToString(),
            name = "Demo",
            biotech = Mathf.FloorToInt(UnityEngine.Random.Range(0, 101)),
            engineering = Mathf.FloorToInt(UnityEngine.Random.Range(0, 101)),
            psychics = Mathf.FloorToInt(UnityEngine.Random.Range(0, 101)),
            weapons = Mathf.FloorToInt(UnityEngine.Random.Range(0, 101))
        };
    }
}
