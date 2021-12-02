using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InstantiatedTokens : MonoBehaviour
{
    public static List<GameObject> instantiatedObjects;

<<<<<<< HEAD
    public static string cardId = "";
    public static int cardWeapons = 0;
    public static int cardEngineering = 0;
    public static int cardBiotech = 0;
    public static int cardPsychics = 0;
=======
    public static int cardWeapons;
    public static int cardEngineering;
    public static int cardBiotech;
    public static int cardPsychics;
>>>>>>> parent of 7691664b (New changes and updates in Grid - WebbridgeGl - Battle Arean - Mint Heroes)

    void Awake()
    {
        instantiatedObjects = new List<GameObject>();
    }
}
