using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[System.Serializable]
public class BattleState
{
    public Token your_hero;
    public int heroes_waiting;

    public static BattleState CreateFromJSON(string jsonString)
    {
        return JsonUtility.FromJson<BattleState>(jsonString);
    }
}
