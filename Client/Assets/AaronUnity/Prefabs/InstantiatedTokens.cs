using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InstantiatedTokens : MonoBehaviour
{
    public static List<GameObject> instantiatedObjects;

    public static string cardId = "";
    public static int cardWeapons = 0;
    public static int cardEngineering = 0;
    public static int cardBiotech = 0;
    public static int cardPsychics = 0;

    void Awake()
    {
        instantiatedObjects = new List<GameObject>();
    }
}
