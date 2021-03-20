using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class CardRenderer : MonoBehaviour
{
    // Start is called before the first frame update
    public CardData Data;

    public GameObject[] Models;
    public Material[] Variants;
    private int _activeModel;
    public RenderTexture CharacterSlip;
    public Camera CharacterCam;
    public RawImage ImageTarget;

    public Skill Weapons;
    public Skill Engineering;
    public Skill Biotech;
    public Skill Psychics;

    public bool Hidden;

    public bool ShowSkills;


    [Range(1, 100)]
    public int SkillWeapons;

    [Range(1, 100)]
    public int SkillEngineering;

    [Range(1, 100)]
    public int SkillBiotech;

    [Range(1, 100)]
    public int SkillPsychics;

    public void DoUpdate()
    {
        SetVisuals();
        SetSkills();
    }


   

    private void SetModel(int newModel)
    {

        Models[_activeModel].SetActive(false);
        if (Hidden) return;
        Models[newModel].SetActive(true);
        _activeModel = newModel;
    }

    private void SetMaterial(int newVariant)
    {
        Models[_activeModel].GetComponent<SkinnedMeshRenderer>().material = Variants[newVariant];
    }




    private void Update()
    {
       
    }

    private void SetVisuals()
    {
        int modelId = (SkillWeapons * 3 + SkillEngineering * 2 + SkillBiotech + SkillPsychics) % Models.Length;
        int materialId = (SkillWeapons + SkillEngineering + SkillBiotech * 3 + SkillPsychics * 2) % Variants.Length;

        SetModel(modelId);
        SetMaterial(materialId);
    }

    private void SetSkills()
    {
        Weapons.Value.text = SkillWeapons.ToString();
        Engineering.Value.text = SkillEngineering.ToString();
        Biotech.Value.text = SkillBiotech.ToString();
        Psychics.Value.text = SkillPsychics.ToString();
    }
}
