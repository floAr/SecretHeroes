using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[CreateAssetMenu(fileName = "New Card", menuName = "Cards/New")]
public class CardToken : ScriptableObject
{
    public int Id;
    public string cardName;
    public string cardToken;

    public int cardWeapons;
    public int cardEngineering;
    public int cardBiotech;
    public int cardPsychics;
}
