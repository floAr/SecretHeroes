using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class DrawManager : MonoBehaviour
{
    public Camera DrawCamera;
    public CardRenderer Slot1;
    public CardRenderer Slot2;
    public CardRenderer Slot3;

    public void PrepareSlots(Token[] tokens)
    {
        Slot1.SkillWeapons = tokens[0].weapons;
        Slot1.SkillEngineering = tokens[0].engineering;
        Slot1.SkillBiotech = tokens[0].biotech;
        Slot1.SkillPsychics = tokens[0].psychics;
        Slot1.DoUpdate();


        Slot2.SkillWeapons = tokens[1].weapons;
        Slot2.SkillEngineering = tokens[1].engineering;
        Slot2.SkillBiotech = tokens[1].biotech;
        Slot2.SkillPsychics = tokens[1].psychics;
        Slot2.DoUpdate();


        Slot3.SkillWeapons = tokens[2].weapons;
        Slot3.SkillEngineering = tokens[2].engineering;
        Slot3.SkillBiotech = tokens[2].biotech;
        Slot3.SkillPsychics = tokens[2].psychics;
        Slot3.DoUpdate();
    }
}
