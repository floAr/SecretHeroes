using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class InstantiatedTokens : MonoBehaviour
{
    public static List<GameObject> instantiatedObjects;

    public static int cardWeapons;
    public static int cardEngineering;
    public static int cardBiotech;
    public static int cardPsychics;

    void Awake()
    {
        instantiatedObjects = new List<GameObject>();
    }
}
