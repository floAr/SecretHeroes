using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InstantiatedTokens : MonoBehaviour
{
    public static List<GameObject> instantiatedObjects;

<<<<<<< Updated upstream:Client/Assets/AaronUnity/Prefabs/InstantiatedTokens.cs
    public static int cardWeapons;
    public static int cardEngineering;
    public static int cardBiotech;
    public static int cardPsychics;
=======
    public static string cardId = "";
    public static int cardWeapons = 0;
    public static int cardEngineering = 0;
    public static int cardBiotech = 0;
    public static int cardPsychics = 0;
>>>>>>> Stashed changes:Client/Assets/InstantiatedTokens.cs

    void Awake()
    {
        instantiatedObjects = new List<GameObject>();
    }
}
